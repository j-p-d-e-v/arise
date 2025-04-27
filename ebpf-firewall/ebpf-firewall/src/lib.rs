use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub async fn send_firewall_log() {
    todo!();
}

pub async fn load_firewall_rules() {
    todo!();
}
