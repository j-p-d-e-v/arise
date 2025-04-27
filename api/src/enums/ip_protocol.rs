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
