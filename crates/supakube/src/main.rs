mod apply;
mod error;
mod install_operators;
mod operator;
mod services;

use anyhow::Result;

use clap::{Parser, Subcommand};
use kube::Client;

// Images we are using
const KEYCLOAK_IMAGE: &str = "quay.io/keycloak/keycloak:23.0";
const OAUTH2_PROXY_IMAGE: &str = "quay.io/oauth2-proxy/oauth2-proxy:v7.5.1";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub struct Installer {
    /// Run a cut down version of Bionic for integration testing
    #[arg(long, default_value_t = false)]
    supakube_operator_only: bool,
    /// In which namespace shall we install
    #[arg(long, default_value = "supakube-system")]
    operator_namespace: String,
}

#[derive(Parser)]
pub struct OpenPorts {
    #[arg(long)]
    namespace: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install Supakube into Kubernetes
    Install(Installer),
    /// Open up ports 30000, 30001 i.e. Postgres and Niginx
    OpenPorts(OpenPorts),
    /// Run the Bionic Kubernetes Operator
    Operator {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install(installer) => {
            install_operators::install(installer).await?;
        }
        Commands::Operator {} => {
            operator::operator().await?;
        }
        Commands::OpenPorts(open_ports) => {
            open_dev_ports(open_ports).await?;
        }
    }

    Ok(())
}

const POSTGRES_SERVICE: &str = include_str!("../config/postgres-service.yaml");
const NGINX_SERVICE: &str = include_str!("../config/nginx-service.yaml");

pub async fn open_dev_ports(installer: &crate::OpenPorts) -> Result<()> {
    let client = Client::try_default().await?;

    // Open up the postgres port to the devcontainer
    println!("🚀 Mapping Postgres to port 30001");
    apply::apply(&client, POSTGRES_SERVICE, Some(&installer.namespace))
        .await
        .unwrap();
    println!("🚀 Mapping Nginx to port 30000");
    apply::apply(&client, NGINX_SERVICE, Some(&installer.namespace)).await?;

    Ok(())
}
