use super::crd::Supakube;
use super::finalizer;
use crate::error::Error;
use crate::services::database;
use crate::services::keycloak;
use crate::services::keycloak_db;
use crate::services::nginx::deploy_nginx;
use crate::services::oauth2_proxy;
use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::api::ListParams;
use kube::Api;
use kube::Client;
use kube::Resource;
use kube::ResourceExt;
use kube_runtime::controller::Action;
use std::{sync::Arc, time::Duration};

/// Context injected with each `reconcile` and `on_error` method invocation.
pub struct ContextData {
    /// Kubernetes client to make Kubernetes API requests with. Required for K8S resource management.
    client: Client,
}

impl ContextData {
    // Constructs a new instance of ContextData.
    //
    // # Arguments:
    // - `client`: A Kubernetes client to make Kubernetes REST API requests with.
    // Resources will be created and deleted with this client.
    pub fn new(client: Client) -> Self {
        ContextData { client }
    }
}

/// Action to be taken upon an `Supakube` resource during reconciliation
enum SupakubeAction {
    /// Create the subresources, this includes spawning `n` pods with Bionic service
    Create,
    /// Delete all subresources created in the `Create` phase
    Delete,
    /// CRD version has chnaged, upgrade the installation.
    Upgrade,
    /// This `Bionic` resource is in desired state and requires no actions to be taken
    NoOp,
}

pub async fn reconcile(
    supakube: Arc<Supakube>,
    context: Arc<ContextData>,
) -> Result<Action, Error> {
    let client: Client = context.client.clone(); // The `Client` is shared -> a clone from the reference is obtained

    let namespace: String = supakube.namespace().unwrap_or("default".to_string());

    let development = supakube.spec.development.unwrap_or_default();
    let name = supakube.name_any();

    let supakube_version = get_current_supakube_version(&client, &namespace).await?;

    // Performs action as decided by the `determine_action` function.
    match determine_action(&supakube, supakube_version) {
        SupakubeAction::Create | SupakubeAction::Upgrade => {
            // Creates a deployment with `n` Bionic service pods, but applies a finalizer first.
            // Finalizer is applied first, as the operator might be shut down and restarted
            // at any time, leaving subresources in intermediate state. This prevents leaks on
            // the `Bionic` resource deletion.

            // Apply the finalizer first. If that fails, the `?` operator invokes automatic conversion
            // of `kube::Error` to the `Error` defined in this crate.
            finalizer::add(client.clone(), &name, &namespace).await?;

            let override_db_password = if development {
                Some("testpassword".to_string())
            } else {
                None
            };

            // The databases
            database::deploy_app_database(
                &client,
                &namespace,
                &supakube.spec.app_name,
                &None,
                &override_db_password,
            )
            .await?;
            keycloak_db::deploy_keycloak_database(
                &client,
                &namespace,
                supakube.spec.keycloak_db_disk_size,
            )
            .await?;

            keycloak::deploy_keycloak(&client, &supakube.spec.hostname_url, &namespace)
                .await
                .unwrap();
            oauth2_proxy::deploy_oauth2_proxy(
                &client,
                &supakube.spec.hostname_url,
                &supakube.spec.app_name,
                &namespace,
            )
            .await
            .unwrap();

            deploy_nginx(&client, &namespace).await.unwrap();

            Ok(Action::requeue(Duration::from_secs(10)))
        }
        SupakubeAction::Delete => {
            keycloak::delete(client.clone(), &namespace).await?;
            keycloak_db::delete(client.clone(), &namespace).await?;
            oauth2_proxy::delete(client.clone(), &namespace).await?;

            database::delete(client.clone(), &namespace, &supakube.spec.app_name).await?;

            // Once the deployment is successfully removed, remove the finalizer to make it possible
            // for Kubernetes to delete the `Bionic` resource.
            finalizer::delete(client, &name, &namespace).await?;
            Ok(Action::await_change()) // Makes no sense to delete after a successful delete, as the resource is gone
        }
        // The resource is already in desired state, do nothing and re-check after 10 seconds
        SupakubeAction::NoOp => Ok(Action::requeue(Duration::from_secs(10))),
    }
}

/// If we already have a deployment, get the version we are running.
/// We can do this by getting the bionic pod by name
async fn get_current_supakube_version(
    client: &Client,
    namespace: &str,
) -> Result<Option<String>, Error> {
    // Get the API for Pod resources in the specified namespace
    let pods: Api<Pod> = Api::namespaced(client.clone(), namespace);
    let lp = ListParams::default().labels("app=bionic-gpt"); // for this app only

    for p in pods.list(&lp).await? {
        if let Some(spec) = p.spec {
            for container in spec.containers {
                if let Some(env_vars) = container.env {
                    for env_var in env_vars {
                        if env_var.name == "VERSION" {
                            return Ok(env_var.value);
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Resources arrives into reconciliation queue in a certain state. This function looks at
/// the state of given `Bionic` resource and decides which actions needs to be performed.
/// The finite set of possible actions is represented by the `BionicAction` enum.
///
/// # Arguments
/// - `echo`: A reference to `Supakube` being reconciled to decide next action upon.
fn determine_action(supakube: &Supakube, version: Option<String>) -> SupakubeAction {
    let version = if let Some(version) = version {
        version
    } else {
        "".to_string()
    };

    if supakube.meta().deletion_timestamp.is_some() {
        SupakubeAction::Delete
    } else if supakube
        .meta()
        .finalizers
        .as_ref()
        .map_or(true, |finalizers| finalizers.is_empty())
    {
        SupakubeAction::Create
    } else if supakube.spec.version != version {
        tracing::info!(
            "Upgrade detected from {} to {}",
            version,
            supakube.spec.version
        );
        SupakubeAction::Upgrade
    } else {
        SupakubeAction::NoOp
    }
}

/// Actions to be taken when a reconciliation fails - for whatever reason.
/// Prints out the error to `stderr` and requeues the resource for another reconciliation after
/// five seconds.
///
/// # Arguments
/// - `supakube`: The erroneous resource.
/// - `error`: A reference to the `kube::Error` that occurred during reconciliation.
/// - `_context`: Unused argument. Context Data "injected" automatically by kube-rs.
pub fn on_error(supakube: Arc<Supakube>, error: &Error, _context: Arc<ContextData>) -> Action {
    eprintln!("Reconciliation error:\n{:?}.\n{:?}", error, supakube);
    Action::requeue(Duration::from_secs(5))
}
