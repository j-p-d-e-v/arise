use anyhow::Context as _;
use aya::{
    programs::{Xdp, XdpFlags},
    Pod,
};
use clap::Parser;
use network_types::ip::IpProto;
#[rustfmt::skip]
use log::{debug, warn};
use aya::maps::{
    lpm_trie::{Key, LpmTrie},
    HashMap,
};
use ebpf_firewall::{
    config::{ApiServerConfig, AppConfig},
    get_protocol, load_firewall_rules, FirewallRuleData,
};
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

    #[clap(short, long, default_value = "Config.toml")]
    config_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    let config_path: String = opt.config_path;

    env_logger::init();

    let app_config: AppConfig = match AppConfig::load(Some(config_path)) {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error.to_string()),
    };
    let api_server_config: ApiServerConfig = app_config.api_server;

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
    let iface: String = opt.iface;
    let program: &mut Xdp = ebpf.program_mut("ebpf_firewall").unwrap().try_into()?;
    program.load()?;
    program.attach(&iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    for map in ebpf.maps_mut() {
        if map.0 == "FIREWALL_RULES" {
            let mut firewall_rules: LpmTrie<_, [u8; 4], Rule> = LpmTrie::try_from(map.1)?;
            let data: Vec<FirewallRuleData> = load_firewall_rules(api_server_config.clone())
                .await
                .unwrap();
            let mut cidrs: Vec<u16> = Vec::new();

            for item in data {
                if !cidrs.contains(&item.cidr) {
                    cidrs.push(item.cidr.clone());
                }
                let key = Key::new(item.cidr as u32, item.ip);
                let rule: Rule = Rule {
                    from_port: item.from_port,
                    to_port: item.to_port,
                    status: item.status,
                    protocol: get_protocol(item.protocol),
                };
                firewall_rules.insert(&key, &rule, 0).unwrap();
            }
        } else if map.0 == "FIREWALL_CIDRS" {
            let mut firewall_cidrs: HashMap<_, u16, u16> = HashMap::try_from(map.1).unwrap();
            let data: Vec<FirewallRuleData> = load_firewall_rules(api_server_config.clone())
                .await
                .unwrap();
            for item in data {
                if let Err(_) = firewall_cidrs.get(&item.cidr, 0) {
                    firewall_cidrs.insert(&item.cidr, item.cidr, 0).unwrap();
                }
            }
        }
    }

    //    let firewall_cidrs_map = ebpf.map_mut("FIREWALL_CIDRS").unwrap();
    //   drop(firewall_cidrs_map);

    let ctrl_c = signal::ctrl_c();
    println!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    println!("Exiting...");

    Ok(())
}
