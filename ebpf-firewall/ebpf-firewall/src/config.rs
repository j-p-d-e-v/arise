use std::{fs::File, io::Read};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiServerConfig {
    pub base_url: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_server: ApiServerConfig,
}

impl AppConfig {
    pub fn load(path: Option<String>) -> Result<AppConfig, anyhow::Error> {
        let path: String = path.unwrap_or("Config.toml".to_string());
        match File::options().read(true).open(path) {
            Ok(mut file) => {
                let mut buf: String = String::new();
                if let Err(error) = file.read_to_string(&mut buf) {
                    return Err(anyhow!(error.to_string()));
                }
                match toml::from_str::<AppConfig>(&buf) {
                    Ok(config) => Ok(config),
                    Err(error) => Err(anyhow!(error.to_string())),
                }
            }
            Err(error) => Err(anyhow!(error.to_string())),
        }
    }
}
