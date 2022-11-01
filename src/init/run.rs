use crate::{cli::InitArgs, config::Config, utils::types::KonectorResult};

pub async fn run_init(args: &InitArgs) -> KonectorResult {
    let username = &args.username;
    let interval = &args.interval;
    let usernames = &[username.to_string()];
    let config = Config::new(usernames, interval);
    config.save().await
}
