use std::{fs::File, io::Read};

use reqwest;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Clone, Serialize)]
pub struct CommandExecutionRequestForm {
    pub command: String,
    pub args: String,
    pub tgid: u32,
    pub pid: u32,
    pub gid: u32,
    pub uid: u32,
}

pub async fn send_log(base_url: String, data: CommandExecutionRequestForm) -> Result<(), String> {
    let url: String = format!("{}/command-execution/log", base_url);
    match reqwest::Client::builder().build() {
        Ok(client) => match client.post(url).json(&data).send().await {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("[REQUEST ERROR] send_log: {}", error.to_string())),
        },
        Err(error) => Err(format!("[REQUEST ERROR] send_log: {}", error.to_string())),
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct ApiServerConfig {
    pub base_url: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub api_server: ApiServerConfig,
}

impl AppConfig {
    pub fn load(path: Option<String>) -> Result<AppConfig, String> {
        let path: String = if let Some(value) = path {
            value
        } else {
            "Config.toml".to_string()
        };
        match File::options().read(true).open(path) {
            Ok(mut file) => {
                let mut content = String::new();
                if let Err(error) = file.read_to_string(&mut content) {
                    return Err(format!(
                        "[APP_CONFIG ERROR] read_to_string: config is empty, length is {}",
                        error.to_string()
                    ));
                }

                match toml::from_str::<AppConfig>(&content) {
                    Ok(config) => Ok(config),
                    Err(error) => Err(format!(
                        "[APP_CONFIG ERROR] toml::from_str: {}",
                        error.to_string()
                    )),
                }
            }
            Err(error) => Err(format!("[APP_CONFIG ERROR] load: {}", error.to_string())),
        }
    }
}
