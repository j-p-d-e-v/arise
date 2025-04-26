#![no_std]
#![no_main]

use core::mem;

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::{lpm_trie::Key, HashMap, LpmTrie},
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
    from_port: Option<u16>,
    to_port: Option<u16>,
    status: u8,
}

#[map]
static FIREWALL_RULES: LpmTrie<[u8; 4], Rule> = LpmTrie::with_max_entries(1024, 0);

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

fn try_ebpf_firewall(ctx: XdpContext) -> Result<u32, ()> {
    //info!(&ctx, "received a packet");
    let key = Key::new(32, [192, 168, 211, 1]);
    if let Err(_) = FIREWALL_RULES.insert(
        &key,
        &Rule {
            from_port: None,
            to_port: None,
            status: 1,
        },
        0,
    ) {
        return Err(());
    }
    let eth_hdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
    match unsafe { *eth_hdr }.ether_type {
        EtherType::Ipv4 => {
            let ipv4_hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
            let source_addr = unsafe { (*ipv4_hdr).src_addr() };
            let total_len = unsafe { *ipv4_hdr }.total_len();
            let source_ipv4: [u8; 4] = source_addr.octets();
            let protocol: IpProto = unsafe { (*ipv4_hdr).proto };
            info!(&ctx, "Protocol: {} ", procotol_to_string(&protocol));
            info!(
                &ctx,
                "IP: {}.{}.{}.{}", source_ipv4[0], source_ipv4[1], source_ipv4[2], source_ipv4[3]
            );
            info!(&ctx, "Total Len: {}", total_len);
            let source_key = Key::new(32, source_ipv4.clone());
            if let Some(item) = FIREWALL_RULES.get(&source_key) {
                info!(
                    &ctx,
                    "Rule: {} - {} [{}]",
                    item.from_port.unwrap_or(0),
                    item.to_port.unwrap_or(0),
                    item.status
                );
            }
            match &protocol {
                &IpProto::Tcp => {
                    let tcp_hdr: *const TcpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + Ipv4Hdr::LEN) }?;
                    let port: u16 = u16::from_be(unsafe { (*tcp_hdr).source });
                    info!(&ctx, "Source Port: {}", port);
                }
                &IpProto::Udp => {
                    let udp_hdr: *const UdpHdr =
                        unsafe { ptr_at(&ctx, EthHdr::LEN + UdpHdr::LEN) }?;
                    let port: u16 = u16::from_be(unsafe { (*udp_hdr).source() });
                    info!(&ctx, "Source Port: {}", port);
                }
                &IpProto::Icmp => {
                    return Ok(xdp_action::XDP_DROP);
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
