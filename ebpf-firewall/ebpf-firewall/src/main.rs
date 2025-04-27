use anyhow::Context as _;
use aya::{
    programs::{Xdp, XdpFlags},
    Pod,
};
use clap::Parser;
use network_types::ip::IpProto;
#[rustfmt::skip]
use log::{debug, warn};
use aya::maps::lpm_trie::{Key, LpmTrie};
use tokio::signal;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Rule {
    pub from_port: Option<u16>,
    pub to_port: Option<u16>,
    pub status: bool,
    pub protocol: IpProto,
}
unsafe impl Pod for Rule {}
#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "eth0")]
    iface: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {ret}");
    }
    println!("iface: {}", opt.iface);
    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/ebpf-firewall"
    )))?;
    if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {e}");
    }
    let Opt { iface } = opt;
    let program: &mut Xdp = ebpf.program_mut("ebpf_firewall").unwrap().try_into()?;
    program.load()?;
    program.attach(&iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;
    let firewall_rules_map = ebpf.map_mut("FIREWALL_RULES").unwrap();
    let mut firewall_rules: LpmTrie<_, [u8; 4], Rule> = LpmTrie::try_from(firewall_rules_map)?;
    let key = Key::new(24, [192, 168, 211, 0]);
    firewall_rules
        .insert(
            &key,
            &Rule {
                from_port: None,
                to_port: None,
                status: false,
                protocol: IpProto::Icmp,
            },
            0,
        )
        .unwrap();

    let ctrl_c = signal::ctrl_c();
    println!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    println!("Exiting...");

    Ok(())
}
