use clap::Parser;
use cli::Cli;
use utils::types::KonectorResult;
mod cli;
mod config;
mod github;
mod init;
mod service;
mod ssh;
mod systemctl;
mod utils;

#[tokio::main]
async fn main() -> KonectorResult {
    let cli = Cli::parse();
    cli.run().await
}
