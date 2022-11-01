use std::time::Duration;

use crate::{config::Config, github, ssh, utils::types::KonectorResult};

pub async fn run() -> KonectorResult {
    loop {
        let config = Config::load().await?;
        let wait_minutes = std::cmp::min(config.interval, 1);

        let data = github::fetch_users(config.usernames).await?;
        let keys: Vec<String> = data.into_iter().map(|data| data.key).collect();

        ssh::save_keys(&keys).await?;
        tokio::time::sleep(Duration::from_secs(wait_minutes * 60)).await;
    }
}
