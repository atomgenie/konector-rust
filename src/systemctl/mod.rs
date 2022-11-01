use std::os::unix::prelude::PermissionsExt;

use tokio::io::AsyncWriteExt;

use crate::{cli::InitServiceArgs, utils::types::KonectorResult};

const SERVICE_PATH: &str = "/etc/systemd/system/konector.service";

pub async fn create_service(args: &InitServiceArgs) -> KonectorResult {
    let user = match &args.user {
        Some(user) => user.to_string(),
        None => std::env::var("USER")?,
    };

    let mut file = tokio::fs::File::create(SERVICE_PATH).await?;
    let txt = build_systemctl_config(&user);
    file.write(txt.as_bytes()).await?;
    file.sync_all().await?;

    tokio::fs::set_permissions(SERVICE_PATH, std::fs::Permissions::from_mode(0o777)).await?;

    Ok(())
}

fn build_systemctl_config(user: &String) -> String {
    let config = format!(
        r#"
[Unit]
Description=Konector daemon
After=network-online.target
StartLimitIntervalSec=0

[Service]
Type=simple
User={}
ExecStart=/usr/bin/env konector service
Restart=on-failure
RestartSec=30
# Configures the time to wait before service is stopped forcefully.

[Install]
WantedBy=multi-user.target
"#,
        user
    );

    config
}
