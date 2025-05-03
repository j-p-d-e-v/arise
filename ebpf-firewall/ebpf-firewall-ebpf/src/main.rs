#![no_std]
#![no_main]

use core::mem;

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::{lpm_trie::Key, HashMap, LpmTrie, PerfEventArray},
    programs::XdpContext,
};
use aya_log_ebpf::info;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
    tcp::TcpHdr,
    udp::UdpHdr,
};
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Rule {
    pub from_port: Option<u16>,
    pub to_port: Option<u16>,
    pub status: bool,
    pub protocol: IpProto,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FirewallLog {
    pub ip: [u8; 4],
    pub port: u16,
    pub protocol: u8,
    pub status: u8,
}

#[map]
static FIREWALL_RULES: LpmTrie<[u8; 4], Rule> = LpmTrie::with_max_entries(1024, 0);

#[map]
static FIREWALL_CIDRS: HashMap<u16, u16> = HashMap::with_max_entries(32, 0);

#[map]
static FIREWALL_LOG: PerfEventArray<FirewallLog> = PerfEventArray::new(0);

#[xdp]
pub fn ebpf_firewall(ctx: XdpContext) -> u32 {
    match try_ebpf_firewall(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

fn procotol_to_string(protocol: &IpProto) -> &str {
    match protocol {
        &IpProto::Tcp => "Tcp",
        &IpProto::Udp => "Udp",
        &IpProto::Icmp => "Icmp",
        _ => "Undefined",
    }
}

/// Check if source port is in port range or exact
/// if from_port and source_port is given, it will do port range checking
/// if only from_port is given, it will do exact port checking
/// if match, it will return true else false
fn check_port(source_port: u16, from_port: Option<u16>, to_port: Option<u16>) -> bool {
    if let Some(from_port) = from_port {
        if let Some(to_port) = to_port {
            if from_port <= source_port && to_port >= source_port {
                return true;
            }
        } else {
            if from_port == source_port {
                return true;
            }
        }
    }
    false
}

/// Check if the firewall rule tells source is allowed or denied.
/// If status is true, its allowed.
/// If status is false, its denied.
fn checked_firewall_rule(
    ctx: &XdpContext,
    protocol: &IpProto,
    source_ipv4: [u8; 4],
    source_port: Option<u16>,
) -> aya_ebpf::bindings::xdp_action::Type {
    let mut rule: Option<&Rule> = None;
    let mut status: bool = true;

    for value in 0..33 {
        let index: u16 = 32 - value;
        if let Some(prefix_length) = unsafe { FIREWALL_CIDRS.get(&index) } {
            let source_key = Key::new(*prefix_length as u32, source_ipv4.clone());
            if let Some(item) = FIREWALL_RULES.get(&source_key) {
                rule = Some(item);
                break;
            }
        }
    }
    if let Some(rule) = rule {
        if protocol == &rule.protocol {
            if let Some(source_port) = source_port {
                if check_port(source_port, rule.from_port, rule.to_port) {
                    status = rule.status;
                }
            } else {
                status = rule.status;
            }
        }
    }
    if status {
        return xdp_action::XDP_PASS;
    }
    info!(
        ctx,
        "[DROPPED] Protocol: {}, IP Address: {}.{}.{}.{}, Port:{}",
        procotol_to_string(protocol),
        source_ipv4[0],
        source_ipv4[1],
        source_ipv4[2],
        source_ipv4[3],
        source_port.unwrap_or(0)
    );
    FIREWALL_LOG.output(
        ctx,
        &FirewallLog {
            ip: source_ipv4,
            status: 0,
            port: source_port.unwrap_or(0),
            protocol: *protocol as u8,
        },
        0,
    );
    xdp_action::XDP_DROP
}

fn try_ebpf_firewall(ctx: XdpContext) -> Result<u32, ()> {
    let eth_hdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
    match unsafe { *eth_hdr }.ether_type {
        EtherType::Ipv4 => {
            let ipv4_hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let source_addr = unsafe { (*ipv4_hdr).src_addr() };
            //            let total_len = unsafe { *ipv4_hdr }.total_len();
            let source_ipv4: [u8; 4] = source_addr.octets();
            let protocol: IpProto = unsafe { (*ipv4_hdr).proto };
            match &protocol {
                &IpProto::Tcp => {
                    let tcp_hdr: *const TcpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN) }?;
                    let port: u16 = u16::from_be(unsafe { (*tcp_hdr).source });
                    return Ok(checked_firewall_rule(
                        &ctx,
                        &protocol,
                        source_ipv4,
                        Some(port),
                    ));
                }
                &IpProto::Udp => {
                    let udp_hdr: *const UdpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + UdpHdr::LEN) }?;
                    let port: u16 = u16::from_be(unsafe { (*udp_hdr).source() });
                    return Ok(checked_firewall_rule(
                        &ctx,
                        &protocol,
                        source_ipv4,
                        Some(port),
                    ));
                }
                &IpProto::Icmp => {
                    return Ok(checked_firewall_rule(&ctx, &protocol, source_ipv4, None));
                }
                _ => {}
            };
        }
        _ => {}
    }

    Ok(xdp_action::XDP_PASS)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link_section = "license"]
#[no_mangle]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
