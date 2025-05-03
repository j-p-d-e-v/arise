use anyhow::Context as _;
use aya::programs::{Xdp, XdpFlags};
use clap::Parser;
#[rustfmt::skip]
use log::{debug, warn};
use aya::maps::{lpm_trie::LpmTrie, HashMap};
use ebpf_firewall::{
    api::Api,
    config::{ApiServerConfig, AppConfig, EbpfConfig},
    maps::{configure_firewall_cidrs, configure_firewall_log, configure_firewall_rules},
    rule::Rule,
};
use tokio::signal;

#[derive(Debug, Parser)]
struct Opt {
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
    let ebpf_config: EbpfConfig = app_config.ebpf;
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
    println!("Interface: {}", ebpf_config.interface);
    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    let layer: u8 = ebpf_config.layer;
    let iface: String = ebpf_config.interface;
    let api_server_config: ApiServerConfig = app_config.api_server;
    let api: Api = Api::new(api_server_config.clone());
    let fwr_update_duration = ebpf_config.fwr_update_duration;
    tokio::task::spawn(async move {
        let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
            env!("OUT_DIR"),
            "/ebpf-firewall"
        )))
        .unwrap();
        if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
            // This can happen if you remove all log statements from your eBPF program.
            warn!("failed to initialize eBPF logger: {e}");
        }
        let program: &mut Xdp = ebpf
            .program_mut("ebpf_firewall")
            .unwrap()
            .try_into()
            .unwrap();
        program.load().unwrap();
        program.attach(&iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE").unwrap();

        let firewall_log_map = ebpf.take_map("FIREWALL_LOG").unwrap();
        if let Err(error) = configure_firewall_log(&api, firewall_log_map).await {
            panic!("{:?}", error);
        }
        loop {
            for map in ebpf.maps_mut() {
                let map_key: &str = map.0;
                if map_key == "FIREWALL_RULES" {
                    let mut firewall_rules: LpmTrie<_, [u8; 4], Rule> =
                        match LpmTrie::try_from(map.1) {
                            Ok(value) => value,
                            Err(error) => panic!("{:?}", error),
                        };

                    if let Err(error) =
                        configure_firewall_rules(&api, layer, &mut firewall_rules).await
                    {
                        panic!("{:?}", error);
                    }
                } else if map_key == "FIREWALL_CIDRS" {
                    let mut firewall_cidrs: HashMap<_, u16, u16> = match HashMap::try_from(map.1) {
                        Ok(value) => value,
                        Err(error) => panic!("{:?}", error),
                    };
                    if let Err(error) =
                        configure_firewall_cidrs(&api, layer, &mut firewall_cidrs).await
                    {
                        panic!("{:?}", error);
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(fwr_update_duration));
        }
    });

    let ctrl_c = signal::ctrl_c();
    println!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    println!("Exiting...");

    Ok(())
}
