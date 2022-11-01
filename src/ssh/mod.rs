use tokio::io::AsyncWriteExt;

use crate::utils::{self, types::KonectorResult};

const SSH_PATH: &str = ".ssh/authorized_keys";

pub async fn save_keys(keys: &Vec<String>) -> KonectorResult {
    let home_path = utils::get_home()?;
    let path = format!("{}/{}", home_path, SSH_PATH);

    let mut file = tokio::fs::File::create(path).await?;

    for key in keys {
        file.write(key.as_bytes()).await?;
        file.write(b"\n").await?;
    }

    file.sync_all().await?;

    Ok(())
}
