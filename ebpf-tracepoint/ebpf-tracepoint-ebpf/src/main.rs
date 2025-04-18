#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::{bpf_probe_read_user, bpf_probe_read_user_str_bytes},
    macros::{map, tracepoint},
    maps::perf::PerfEventArray,
    programs::TracePointContext,
    EbpfContext,
};
use ebpf_tracepoint_common::{ARGV_LEN, ARGV_OFFSET, COMMAND_LEN};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandInfo {
    pub command_len: usize,
    pub argvs_offset: [usize; ARGV_OFFSET],
    pub command: [u8; COMMAND_LEN],
    pub argvs: [[u8; ARGV_LEN]; ARGV_OFFSET],
    pub tgid: u32,
    pub pid: u32,
    pub gid: u32,
    pub uid: u32,
}
#[map]
static COMMAND_EVENTS: PerfEventArray<CommandInfo> = PerfEventArray::new(0);

#[tracepoint]
pub fn ebpf_tracepoint(ctx: TracePointContext) -> u32 {
    match try_ebpf_tracepoint(ctx) {
        Ok(ret) => ret,
        Err(_) => 1,
    }
}

fn try_ebpf_tracepoint(ctx: TracePointContext) -> Result<u32, i64> {
    let command_ptr = unsafe { ctx.read_at::<*const u8>(16)? };
    let mut command_buf: [u8; COMMAND_LEN] = [0u8; COMMAND_LEN];
    let command: &[u8] = unsafe { bpf_probe_read_user_str_bytes(command_ptr, &mut command_buf)? };

    let mut argvs_len: [usize; ARGV_OFFSET] = [0usize; ARGV_OFFSET];
    let mut argvs_buf: [[u8; ARGV_LEN]; ARGV_OFFSET] = [[0u8; ARGV_LEN]; ARGV_OFFSET];
    let argv = unsafe { ctx.read_at::<*const *const u8>(24)? };

    for i in 0..ARGV_OFFSET {
        let argv_ptr: *const u8 = unsafe { bpf_probe_read_user(argv.offset(i as isize + 1))? };
        if argv_ptr.is_null() {
            break;
        }
        let argv: &[u8] =
            unsafe { bpf_probe_read_user_str_bytes(argv_ptr, &mut argvs_buf[i as usize])? };
        let argv_len = argv.len();
        argvs_len[i as usize] = if argv_len >= 32 { 32 } else { argv_len };
    }

    let tgid: u32 = ctx.tgid();
    let gid: u32 = ctx.gid();
    let pid: u32 = ctx.pid();
    let uid: u32 = ctx.uid();

    COMMAND_EVENTS.output(
        &ctx,
        &CommandInfo {
            command_len: command.len(),
            argvs_offset: argvs_len,
            command: command_buf,
            argvs: argvs_buf,
            uid: uid,
            gid: gid,
            pid: pid,
            tgid: tgid,
        },
        0,
    );

    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link_section = "license"]
#[no_mangle]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
