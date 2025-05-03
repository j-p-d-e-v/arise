use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::protocol::IpProtocol;
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct FirewallLog {
    pub ip: [u8; 4],
    pub port: u16,
    pub protocol: u8,
    pub status: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FirewallLogData {
    pub ip: [u8; 4],
    pub protocol: IpProtocol,
    pub port: Option<u16>,
    pub status: bool,
}
