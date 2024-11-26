mod apply;
mod error;
mod operator;
mod services;
mod install_operators;

use anyhow::Result;

use clap::{Parser, Subcommand};

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

#[derive(Subcommand)]
pub enum Commands {
    /// Install Bionic into Kubernetes
    Install(Installer),
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
    }

    Ok(())
}
