use aya::programs::TracePoint;
#[rustfmt::skip]
use log::{debug, warn};
use aya::{maps::perf::AsyncPerfEventArray, util::online_cpus, Pod};
use bytemuck::{Pod as BPod, Zeroable};
use bytes::BytesMut;
use ebpf_tracepoint_common::{ARGV_LEN, ARGV_OFFSET};
use tokio::signal;
#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, BPod)]
pub struct CommandInfo {
    pub command_len: usize,
    pub argvs_offset: [usize; ARGV_OFFSET],
    pub command: [u8; 64],
    pub argvs: [[u8; ARGV_LEN]; ARGV_OFFSET],
}
unsafe impl Pod for CommandInfo {}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {}", ret);
    }

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/ebpf-tracepoint"
    )))?;
    if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }
    let program: &mut TracePoint = ebpf.program_mut("ebpf_tracepoint").unwrap().try_into()?;
    program.load()?;
    program.attach("syscalls", "sys_enter_execve")?;

    let mut perf_command_events =
        AsyncPerfEventArray::try_from(ebpf.take_map("COMMAND_EVENTS").unwrap())?;

    for cpu_id in online_cpus().map_err(|(_, error)| error)? {
        let mut buf = perf_command_events.open(cpu_id, None)?;

        tokio::task::spawn(async move {
            let mut buffers = (0..10)
                .map(|_| BytesMut::with_capacity(1024))
                .collect::<Vec<_>>();

            loop {
                match buf.read_events(&mut buffers).await {
                    Ok(events) => {
                        for i in 0..events.read {
                            let buf = &mut buffers[i];
                            let ptr = buf.as_ptr() as *const CommandInfo;
                            let info = unsafe { ptr.read_unaligned() };
                            let command_str =
                                String::from_utf8_lossy(&info.command[..info.command_len]);
                            let mut argsv_str: String = "".to_string();
                            for i in 0..4 {
                                let argv_len: usize = info.argvs_offset[i as usize];
                                if argv_len == 0 {
                                    break;
                                }
                                let argv_buf: [u8; ARGV_LEN] = info.argvs[i as usize];
                                argsv_str.push_str(&String::from_utf8_lossy(&argv_buf[..argv_len]));
                                argsv_str.push_str(" ");
                            }
                            debug!("Command: {} {}", command_str, argsv_str.trim_end());
                        }
                    }
                    Err(err) => {
                        debug!("panic: {:?}", err);
                    }
                }
            }
        });
    }

    let ctrl_c = signal::ctrl_c();
    println!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    println!("Exiting...");

    Ok(())
}
