use std::sync::Arc;

use anyhow::{anyhow, Error};
use aya::{
    maps::{AsyncPerfEventArray, Map},
    util::online_cpus,
};
use bytes::BytesMut;
use log::warn;

use crate::{
    api::Api,
    log::{FirewallLog, FirewallLogData},
    protocol::{get_protocol_from_u8, IpProtocol},
};

pub async fn configure_firewall_log(
    api: &Api,
    server_ip: &str,
    firewall_log_map: Map,
) -> Result<(), Error> {
    let mut perf_array = AsyncPerfEventArray::try_from(firewall_log_map).unwrap();
    let a_api: Arc<Api> = Arc::new(api.to_owned());
    for cpu_id in online_cpus().unwrap() {
        let mut buf = match perf_array.open(cpu_id, None) {
            Ok(value) => value,
            Err(error) => return Err(anyhow!(error)),
        };

        let shared_api = a_api.clone();
        let server_ip = server_ip.to_string();
        tokio::task::spawn(async move {
            let mut buffers: Vec<_> = (0..10).map(|_| BytesMut::with_capacity(1024)).collect();

            loop {
                match buf.read_events(&mut buffers).await {
                    Ok(events) => {
                        for i in 0..events.read {
                            let buf = &mut buffers[i];
                            let ptr = buf.as_ptr() as *const FirewallLog;
                            let info = unsafe { ptr.read_unaligned() };
                            let protocol = get_protocol_from_u8(info.protocol);
                            let (source_port, dest_port) = if &protocol == &IpProtocol::Icmp {
                                (None, None)
                            } else {
                                (Some(info.source_port), Some(info.dest_port))
                            };
                            let data: FirewallLogData = FirewallLogData {
                                ip: info.ip,
                                server_ip: server_ip.clone(),
                                source_port,
                                dest_port,
                                protocol,
                                status: if info.status == 1 { true } else { false },
                            };
                            if let Err(error) = shared_api.send_firewall_log(data).await {
                                warn!("[FIREWALL LOG]: {:?}", error.to_string());
                            }
                        }
                    }
                    Err(error) => {
                        warn!("[FIREWALL LOG] {:?}", error);
                    }
                };
            }
        });
    }
    Ok(())
}
