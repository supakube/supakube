use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// A rust wrapper around the postrgres operator CRD
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct BootstrapSpec {
    pub initdb: InitDBSpec,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct InitDBSpec {
    pub database: String,
    pub owner: String,
    pub secret: SecretSpec,
    #[serde(rename = "postInitSQL")]
    pub post_init_sql: Option<Vec<String>>,
    #[serde(rename = "postInitApplicationSQL")]
    pub post_init_application_sql: Option<Vec<String>>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct SecretSpec {
    pub name: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct StorageSpec {
    pub size: String,
}

/// Corresponds to the Cluster resource
#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "postgresql.cnpg.io",
    version = "v1",
    kind = "Cluster",
    plural = "clusters",
    derive = "PartialEq",
    namespaced
)]
pub struct ClusterSpec {
    pub instances: i32,
    pub bootstrap: BootstrapSpec,
    pub storage: StorageSpec,
}
