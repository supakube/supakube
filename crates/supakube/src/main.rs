mod apply;
mod error;
mod operator;
mod services;

use anyhow::Result;

// Images we are using
const KEYCLOAK_IMAGE: &str = "quay.io/keycloak/keycloak:23.0";
const OAUTH2_PROXY_IMAGE: &str = "quay.io/oauth2-proxy/oauth2-proxy:v7.5.1";

#[tokio::main]
async fn main() -> Result<()> {

    operator::operator().await?;
    Ok(())
}
