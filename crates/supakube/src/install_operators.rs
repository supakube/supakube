use anyhow::Result;
use k8s_openapi::{
    api::{
        apps::v1::Deployment,
        core::v1::{Namespace, ServiceAccount},
        rbac::v1::{ClusterRole, ClusterRoleBinding, PolicyRule, RoleRef, Subject},
    },
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
};
use kube::{
    api::{ObjectMeta, Patch, PatchParams, PostParams},
    Api, Client, CustomResourceExt, Error,
};
use kube_runtime::{
    conditions,
    wait::{await_condition, Condition},
};
use serde_json::json;

use crate::operator::crd::Supakube;

const CNPG_YAML: &str = include_str!("../config/cnpg-1.22.1.yaml");
const OPERATOR_IMAGE: &str = "ghcr.io/supakube/supakube";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const MANAGER: &str = "supakube-operator";
const OPERATOR_NAME: &str = "supakube-operator";

pub async fn install(installer: &crate::Installer) -> Result<()> {
    println!("🔗 Connecting to the cluster...");
    let client = Client::try_default().await?;
    println!("🔗 Connected");

    println!("🔧 Creating Namespace : {}", &installer.operator_namespace);
    create_namespace(&client, &installer.operator_namespace).await?;

    if !installer.supakube_operator_only {
        install_postgres_operator(&client).await?;
    }

    create_operator(&client, &installer.operator_namespace).await?;
    create_crd(&client).await?;
    create_roles(&client, installer).await?;

    Ok(())
}

async fn create_crd(client: &Client) -> Result<(), Error> {
    println!("Installing Supakube CRD");
    let crd = Supakube::crd();
    let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
    crds.patch(
        "supakubes.supakube.com",
        &PatchParams::apply(MANAGER),
        &Patch::Apply(crd),
    )
    .await?;

    println!("Waiting for Supakube CRD");
    let establish = await_condition(
        crds,
        "supakubes.supakube.com",
        conditions::is_crd_established(),
    );
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish)
        .await
        .unwrap();
    Ok(())
}

async fn create_operator(client: &Client, namespace: &str) -> Result<()> {
    println!("🔧 Installing the Supakube Operator into {}", namespace);
    let app_labels = serde_json::json!({
        "app": OPERATOR_NAME,
    });

    let deployment = serde_json::json!({
        "apiVersion": "apps/v1",
        "kind": "Deployment",
        "metadata": {
            "name": format!("{}-deployment", OPERATOR_NAME),
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
                    "serviceAccountName": format!("{}-service-account", OPERATOR_NAME),
                    "containers": json!([{
                        "name": OPERATOR_NAME,
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
            &format!("{}-deployment", OPERATOR_NAME),
            &PatchParams::apply(MANAGER),
            &Patch::Apply(deployment),
        )
        .await?;

    Ok(())
}

async fn create_roles(client: &Client, installer: &super::Installer) -> Result<()> {
    println!("🔧 Setting up roles");
    let sa_api: Api<ServiceAccount> =
        Api::namespaced(client.clone(), &installer.operator_namespace);
    let service_account = ServiceAccount {
        metadata: ObjectMeta {
            name: Some(format!("{}-service-account", OPERATOR_NAME)),
            namespace: Some(installer.operator_namespace.clone()),
            ..Default::default()
        },
        ..Default::default()
    };
    sa_api
        .patch(
            &format!("{}-service-account", OPERATOR_NAME),
            &PatchParams::apply(MANAGER),
            &Patch::Apply(service_account),
        )
        .await?;
    let role_api: Api<ClusterRole> = Api::all(client.clone());
    let role = ClusterRole {
        metadata: ObjectMeta {
            name: Some(format!("{}-cluster-role", OPERATOR_NAME)),
            ..Default::default()
        },
        rules: Some(vec![PolicyRule {
            api_groups: Some(vec!["*".to_string()]),
            resources: Some(vec!["*".to_string()]),
            verbs: vec!["*".to_string()],
            ..Default::default()
        }]),
        ..Default::default()
    };
    role_api
        .patch(
            &format!("{}-cluster-role", OPERATOR_NAME),
            &PatchParams::apply(MANAGER),
            &Patch::Apply(role),
        )
        .await?;

    // Now the cluster role
    let role_binding_api: Api<ClusterRoleBinding> = Api::all(client.clone());
    let role_binding = ClusterRoleBinding {
        metadata: ObjectMeta {
            name: Some(format!("{}-cluster-role-binding", OPERATOR_NAME)),
            ..Default::default()
        },
        role_ref: RoleRef {
            api_group: "rbac.authorization.k8s.io".to_string(),
            kind: "ClusterRole".to_string(),
            name: format!("{}-cluster-role", OPERATOR_NAME),
        },
        subjects: Some(vec![Subject {
            kind: "ServiceAccount".to_string(),
            name: format!("{}-service-account", OPERATOR_NAME),
            namespace: Some(installer.operator_namespace.clone()),
            ..Default::default()
        }]),
    };
    role_binding_api
        .patch(
            &format!("{}-cluster-role-binding", OPERATOR_NAME),
            &PatchParams::apply(MANAGER),
            &Patch::Apply(role_binding),
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
