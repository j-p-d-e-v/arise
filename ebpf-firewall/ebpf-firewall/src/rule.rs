use aya::Pod;
use network_types::ip::IpProto;
use serde::Deserialize;

use crate::protocol::IpProtocol;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Rule {
    pub from_port: Option<u16>,
    pub to_port: Option<u16>,
    pub status: bool,
    pub cidr: u8,
    pub protocol: IpProto,
}
unsafe impl Pod for Rule {}

#[derive(Clone, Debug, Deserialize)]
pub struct FirewallRuleData {
    pub ip: [u8; 4],
    pub protocol: IpProtocol,
    pub cidr: u16,
    pub from_port: Option<u16>,
    pub to_port: Option<u16>,
    pub status: bool,
}

// Order of attributes is important it should be protocol then ip
// because first 8 bits will be the protocol then ip will be based on cidr
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IpProtoKey {
    pub protocol: u8,
    pub ip: [u8; 4],
}

unsafe impl Pod for IpProtoKey {}
