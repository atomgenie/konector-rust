use clap::{Args, Parser, Subcommand};

use crate::{init, service, systemctl, utils::types::KonectorResult};

#[derive(Parser)]
#[command(author, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Init Konector.
    Init(InitArgs),

    /// Start the service.
    Service,

    /// Init the systemctl service.
    InitService(InitServiceArgs),
}

#[derive(Args)]
pub struct InitArgs {
    /// GitHub username.
    pub username: String,

    /// Interval to refresh every x minutes.
    #[arg(long, default_value_t = 1)]
    pub interval: u64,
}

#[derive(Args)]
pub struct InitServiceArgs {
    /// Configure to user.
    #[arg(long)]
    pub user: Option<String>,
}

impl Cli {
    pub async fn run(&self) -> KonectorResult {
        self.command.run().await
    }
}

impl Commands {
    async fn run(&self) -> KonectorResult {
        match self {
            Commands::Init(args) => init::run_init(args).await,
            Commands::Service => service::run().await,
            Commands::InitService(args) => systemctl::create_service(args).await,
        }
    }
}
