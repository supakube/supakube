use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A Bionic resource.
#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "supakube.com",
    version = "v1",
    kind = "Supakube",
    plural = "supakubes",
    derive = "PartialEq",
    namespaced
)]
pub struct SupakubeSpec {
    pub replicas: i32,
    pub version: String,
    pub install_operators: bool,
    pub app_name: String,
    pub disable_ingress: Option<bool>,
    pub pgadmin: Option<bool>,
    pub observability: Option<bool>,
    pub development: Option<bool>,
    pub testing: Option<bool>,
    pub application_db_disk_size: i32,
    pub keycloak_db_disk_size: i32,
    #[serde(rename = "hostname-url")]
    pub hostname_url: String,
}
