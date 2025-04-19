use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warn,
    Debug,
    Error,
    Trace,
}
impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Warn => write!(f, "warn"),
            Self::Debug => write!(f, "debug"),
            Self::Error => write!(f, "error"),
            Self::Trace => write!(f, "trace"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpServerConfig {
    pub port: u16,
    pub host: String,
    pub workers: usize,
    pub log_level: LogLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaServerConfig {
    pub address: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub http_server: HttpServerConfig,
    pub database_server: DatabaServerConfig,
}

impl AppConfig {
    pub fn load(path: Option<String>) -> Result<Self, String> {
        let path: String = if let Some(value) = path {
            value
        } else {
            "Config.toml".to_string()
        };
        match File::options().read(true).open(path) {
            Ok(mut file) => {
                let mut buf: String = String::new();
                if let Ok(_) = file.read_to_string(&mut buf) {
                    match toml::from_str::<AppConfig>(&buf) {
                        Ok(config) => {
                            return Ok(config);
                        }
                        Err(error) => {
                            return Err(format!("config error: {}", error.to_string()));
                        }
                    }
                }
                Err("config error: unable to load config content".to_string())
            }
            Err(error) => Err(format!("config error: {}", error.to_string())),
        }
    }
}

#[cfg(test)]
mod test_app_config {
    use super::*;

    #[tokio::test]
    async fn test_load_config() {
        let app_config = AppConfig::load(None);
        assert!(app_config.is_ok(), "{:?}", app_config.err());
        let _ = app_config.unwrap();
    }
}
