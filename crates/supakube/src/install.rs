use anyhow::Result;
use k8s_openapi::api::{apps::v1::Deployment, core::v1::Namespace};
use kube::{
    api::{ObjectMeta, Patch, PatchParams, PostParams},
    Api, Client, Error,
};
use kube_runtime::wait::{await_condition, Condition};
use serde_json::json;

const CNPG_YAML: &str = include_str!("../config/cnpg-1.22.1.yaml");
const OPERATOR_IMAGE: &str = "ghcr.io/supakube/supakube";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const MANAGER: &str = "supakube-operator";

pub async fn install(installer: &crate::Installer) -> Result<()> {
    println!("🔗 Connecting to the cluster...");
    let client = Client::try_default().await?;
    println!("🔗 Connected");

    println!("🔧 Creating Namespace : {}", &installer.operator_namespace);
    create_namespace(&client, &installer.operator_namespace).await?;

    if ! installer.supakube_operator_only {
        install_postgres_operator(&client).await?;
    }

    create_bionic_operator(&client, &installer.operator_namespace).await?;

    Ok(())
}

async fn create_bionic_operator(client: &Client, namespace: &str) -> Result<()> {
    println!("🔧 Installing the Supakube Operator into {}", namespace);
    let app_labels = serde_json::json!({
        "app": "supakube-operator",
    });

    let deployment = serde_json::json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": "supakube-operator-deployment",
            "namespace": namespace
        },
        "spec": {
            "replicas": 1,
            "selector": {
                "matchLabels": app_labels
            },
            "template": {
                "metadata": {
                    "labels": app_labels
                },
                "spec": {
                    "serviceAccountName": "supakube-operator-service-account",
                    "containers": json!([{
                        "name": "supakube-operator",
                        "image": format!("{}:{}", OPERATOR_IMAGE, VERSION)
                    }]),
                }
            }
        }
    });

    // Create the deployment defined above
    let deployment_api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    deployment_api
        .patch(
            "supakube-operator-deployment",
            &PatchParams::apply(MANAGER),
            &Patch::Apply(deployment),
        )
        .await?;

    Ok(())
}

async fn install_postgres_operator(client: &Client) -> Result<()> {
    println!("🔧 Installing Cloud Native Postgres Operator (CNPG)");
    super::apply::apply(client, CNPG_YAML, None).await?;

    fn is_deployment_available() -> impl Condition<Deployment> {
        |obj: Option<&Deployment>| {
            if let Some(deployment) = &obj {
                if let Some(status) = &deployment.status {
                    if let Some(phase) = &status.available_replicas {
                        return phase >= &1;
                    }
                }
            }
            false
        }
    }

    println!("Waiting for Cloud Native Postgres Controller Manager");
    let deploys: Api<Deployment> = Api::namespaced(client.clone(), "cnpg-system");
    let establish = await_condition(
        deploys,
        "cnpg-controller-manager",
        is_deployment_available(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(120), establish)
        .await
        .unwrap();

    Ok(())
}

pub async fn create_namespace(client: &Client, namespace: &str) -> Result<Namespace> {
    tracing::info!("Ensuring existence of namespace {}", namespace);
    // Define the API object for Namespace
    let namespaces: Api<Namespace> = Api::all(client.clone());

    // Check if the namespace already exists
    match namespaces.get(namespace).await {
        Ok(existing_ns) => {
            tracing::info!("Namespace {} already exists", namespace);
            Ok(existing_ns)
        }
        Err(Error::Api(err)) if err.code == 404 => {
            tracing::info!("Namespace {} not found, creating", namespace);

            let new_namespace = Namespace {
                metadata: ObjectMeta {
                    name: Some(namespace.to_string()),
                    ..Default::default()
                },
                ..Default::default()
            };

            let ns = namespaces
                .create(&PostParams::default(), &new_namespace)
                .await?;
            tracing::info!("Namespace {} created", namespace);
            Ok(ns)
        }
        Err(e) => {
            tracing::error!(
                "Failed to check existence of namespace {}: {:?}",
                namespace,
                e
            );
            Err(e.into())
        }
    }
}
