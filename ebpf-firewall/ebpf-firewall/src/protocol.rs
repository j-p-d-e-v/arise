use network_types::ip::IpProto;
use serde::{Deserialize, Serialize};
#[derive(Serialize, PartialEq, Eq, Deserialize, Clone, Debug)]
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

pub fn get_protocol(protocol: IpProtocol) -> IpProto {
    match protocol {
        IpProtocol::Tcp => IpProto::Tcp,
        IpProtocol::Udp => IpProto::Udp,
        IpProtocol::Icmp => IpProto::Icmp,
        IpProtocol::Undefined => IpProto::Tcp,
    }
}

pub fn get_protocol_from_u8(value: u8) -> IpProtocol {
    if value == IpProto::Tcp as u8 {
        IpProtocol::Tcp
    } else if value == IpProto::Udp as u8 {
        IpProtocol::Udp
    } else if value == IpProto::Icmp as u8 {
        IpProtocol::Icmp
    } else {
        IpProtocol::Undefined
    }
}
