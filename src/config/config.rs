use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use crate::utils;
use crate::utils::errors::KonectorError;
use crate::utils::types::KonectorResult;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub usernames: Vec<String>,
    pub interval: u64,
}

const DIR_CONFIG: &str = ".konector";
const FILE_CONFIG: &str = "config.json";

impl Config {
    pub fn new(usernames: &[String], interval: &u64) -> Config {
        return Config {
            usernames: usernames.to_vec(),
            interval: *interval,
        };
    }

    pub async fn save(&self) -> KonectorResult {
        let home_path = std::env::var("HOME")?;
        let dir_path = format!("{}/{}", home_path, DIR_CONFIG);

        let path = std::path::Path::new(&dir_path);
        if !path.exists() {
            tokio::fs::create_dir(&dir_path).await?;
        }

        if !path.is_dir() {
            return Err(Box::new(KonectorError::new(
                format!("{} is not a directory", &dir_path).as_str(),
            )));
        }

        let file_path = Config::get_file_config_path()?;
        let json_data = serde_json::to_string(&self)?;

        let mut file = tokio::fs::File::create(file_path).await?;
        file.write(json_data.as_bytes()).await?;
        file.sync_all().await?;

        Ok(())
    }

    pub async fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let file_path = Config::get_file_config_path()?;
        let mut file = tokio::fs::File::open(file_path).await?;

        let mut data = String::new();
        file.read_to_string(&mut data).await?;

        let config: Config = serde_json::from_str(data.as_str())?;
        Ok(config)
    }

    fn get_file_config_path() -> Result<String, Box<dyn std::error::Error>> {
        let home_path = utils::get_home()?;
        Ok(format!("{}/{}/{}", home_path, DIR_CONFIG, FILE_CONFIG))
    }
}
