use reqwest::{self, ClientBuilder};

use crate::{config::ApiServerConfig, log::FirewallLogData, rule::FirewallRuleData};

#[derive(Debug, Clone)]
pub struct Api {
    base_url: String,
}

impl Api {
    pub fn new(api_server_config: ApiServerConfig) -> Self {
        Self {
            base_url: api_server_config.base_url,
        }
    }

    pub async fn send_firewall_log(
        &self,
        data: FirewallLogData,
    ) -> Result<FirewallLogData, anyhow::Error> {
        let client = ClientBuilder::new().build()?;
        let url: String = format!("{}/firewall-log/create", self.base_url);
        let response = client.post(url).json(&data).send().await?;
        let data = response.json::<FirewallLogData>().await?;
        Ok(data)
    }

    pub async fn load_firewall_rules(
        &self,
        layer: u8,
    ) -> Result<Vec<FirewallRuleData>, anyhow::Error> {
        let url: String = format!("{}/firewall-rule/list/{}", self.base_url, layer);
        let response = reqwest::get(url).await?;
        let data = response.json::<Vec<FirewallRuleData>>().await?;
        Ok(data)
    }
}

#[cfg(test)]
pub mod test_api {
    use super::*;
    use crate::config::AppConfig;

    #[tokio::test]
    async fn test_load_firewall_rules() {
        let app_config: Result<AppConfig, anyhow::Error> = AppConfig::load(Some(
            "/mnt/coding/coding/arise/ebpf-firewall/Config.toml".to_string(),
        ));
        assert!(app_config.is_ok(), "{:?}", app_config.err());
        let api_server_config: ApiServerConfig = app_config.unwrap().api_server;

        let api: Api = Api::new(api_server_config);
        let data = api.load_firewall_rules(3).await;
        assert!(data.is_ok(), "{:?}", data.err());
    }
}
