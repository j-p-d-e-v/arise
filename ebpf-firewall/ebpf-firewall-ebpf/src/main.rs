#![no_std]
#![no_main]

use core::mem;

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::{lpm_trie::Key, LpmTrie, PerfEventArray},
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
    pub cidr: u8,
    pub protocol: IpProto,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FirewallLog {
    pub ip: [u8; 4],
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: u8,
    pub status: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IpProtoKey {
    pub protocol: u8,
    pub ip: [u8; 4],
}

#[map]
static FIREWALL_RULES: LpmTrie<IpProtoKey, Rule> = LpmTrie::with_max_entries(1024, 0);

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
/// if from_port and dest_port is given, it will do port range checking
/// if only from_port is given, it will do exact port checking
/// if match, it will return true else false
fn check_port(dest_port: u16, from_port: Option<u16>, to_port: Option<u16>) -> bool {
    if let Some(from_port) = from_port {
        if let Some(to_port) = to_port {
            if from_port <= dest_port && to_port >= dest_port {
                return true;
            }
        } else {
            if from_port == dest_port {
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
    port: (Option<u16>, Option<u16>),
) -> aya_ebpf::bindings::xdp_action::Type {
    let source_port = port.0;
    let dest_port = port.1;
    let mut rule: Option<&Rule> = None;
    let mut status: bool = true;
    let key: IpProtoKey = IpProtoKey {
        protocol: *protocol as u8,
        ip: source_ipv4.clone(),
    };
    for value in 0..33 {
        let _cidr: u8 = 32 - value;
        let prefix_length: u32 = _cidr as u32 + 8;
        let source_key = Key::new(prefix_length, key);
        if let Some(item) = FIREWALL_RULES.get(&source_key) {
            info!(
                ctx,
                "found rule cidr: {}:{} from: {}, to:{}, status: {}, protocol: {}, IP Address: {}.{}.{}.{}",
                item.cidr,
                _cidr,
                item.from_port.unwrap_or(0),
                item.to_port.unwrap_or(0),
                if item.status { "true" } else { "false" },
                item.protocol as u8,
                source_ipv4[0],
                source_ipv4[1],
                source_ipv4[2],
                source_ipv4[3],
            );

            if item.cidr == _cidr && &item.protocol == protocol {
                rule = Some(item);
            }
        }
    }
    if let Some(rule) = rule {
        if let Some(dest_port) = dest_port {
            if check_port(dest_port, rule.from_port, rule.to_port) {
                status = rule.status;
            }

            if protocol == &IpProto::Icmp {
                //     info!(ctx, "i got a destination port which should be none");
            }
        } else {
            status = rule.status;
        }
        if protocol == &IpProto::Icmp {
            //  info!(
            //      ctx,
            //      ">> found rule status: {}, protocol: {}, IP Address: {}.{}.{}.{}",
            //      if status { "true" } else { "false" },
            //      *protocol as u8,
            //      source_ipv4[0],
            //      source_ipv4[1],
            //      source_ipv4[2],
            //      source_ipv4[3],
            //  );
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
        dest_port.unwrap_or(0)
    );
    FIREWALL_LOG.output(
        ctx,
        &FirewallLog {
            ip: source_ipv4,
            status: 0,
            source_port: source_port.unwrap_or(0),
            dest_port: dest_port.unwrap_or(0),
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
                    let source_port: u16 = u16::from_be(unsafe { (*tcp_hdr).source });
                    let dest_port: u16 = u16::from_be(unsafe { (*tcp_hdr).dest });
                    return Ok(checked_firewall_rule(
                        &ctx,
                        &protocol,
                        source_ipv4,
                        (Some(source_port), Some(dest_port)),
                    ));
                }
                &IpProto::Udp => {
                    let udp_hdr: *const UdpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + UdpHdr::LEN) }?;
                    let source_port: u16 = u16::from_be(unsafe { (*udp_hdr).source() });
                    let dest_port: u16 = u16::from_be(unsafe { (*udp_hdr).dest() });
                    return Ok(checked_firewall_rule(
                        &ctx,
                        &protocol,
                        source_ipv4,
                        (Some(source_port), Some(dest_port)),
                    ));
                }
                &IpProto::Icmp => {
                    return Ok(checked_firewall_rule(
                        &ctx,
                        &protocol,
                        source_ipv4,
                        (None, None),
                    ));
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
