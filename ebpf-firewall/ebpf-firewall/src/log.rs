use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::protocol::IpProtocol;
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct FirewallLog {
    pub ip: [u8; 4],
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: u8,
    pub status: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FirewallLogData {
    pub ip: [u8; 4],
    pub server_ip: String,
    pub protocol: IpProtocol,
    pub source_port: Option<u16>,
    pub dest_port: Option<u16>,
    pub status: bool,
}
