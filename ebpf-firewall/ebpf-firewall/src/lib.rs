pub mod config;
use config::{ApiServerConfig, AppConfig};
use network_types::ip::IpProto;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum IpProtocol {
    Tcp,
    Udp,
    Icmp,
    Undefined,
}

impl std::fmt::Display for IpProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tcp => write!(f, "Tcp"),
            Self::Udp => write!(f, "Udp"),
            Self::Icmp => write!(f, "Icmp"),
            Self::Undefined => write!(f, "Undefined"),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FirewallRuleData {
    pub ip: [u8; 4],
    pub protocol: IpProtocol,
    pub cidr: u16,
    pub from_port: Option<u16>,
    pub to_port: Option<u16>,
    pub status: bool,
}

pub fn get_protocol(protocol: IpProtocol) -> IpProto {
    match protocol {
        IpProtocol::Tcp => IpProto::Tcp,
        IpProtocol::Udp => IpProto::Udp,
        IpProtocol::Icmp => IpProto::Icmp,
        IpProtocol::Undefined => IpProto::Tcp,
    }
}

pub async fn send_firewall_log() {
    todo!();
}

pub async fn load_firewall_rules(
    api_server_config: ApiServerConfig,
) -> Result<Vec<FirewallRuleData>, anyhow::Error> {
    let url: String = format!("{}/firewall-rule/list", api_server_config.base_url);
    let response = reqwest::get(url).await?;
    let data = response.json::<Vec<FirewallRuleData>>().await?;
    Ok(data)
}

#[tokio::test]
async fn test_load_firewall_rules() {
    let app_config: Result<AppConfig, anyhow::Error> = AppConfig::load(Some(
        "/mnt/coding/coding/arise/ebpf-firewall/Config.toml".to_string(),
    ));
    assert!(app_config.is_ok(), "{:?}", app_config.err());
    let api_server_config: ApiServerConfig = app_config.unwrap().api_server;

    let data = load_firewall_rules(api_server_config).await;
    assert!(data.is_ok(), "{:?}", data.err());
}
