#![no_std]
#![feature(lang_items, core_intrinsics)]

use core::panic::PanicInfo;

use core::fmt::Write;
use sel4_start::{self, DebugOutHandle};
use selfe_sys::{seL4_BootInfo, seL4_CapInitThreadTCB, seL4_TCB_Suspend};

fn main() {
    #[cfg(target_arch = "arm")]
    let arch = "arm";
    #[cfg(target_arch = "x86_64")]
    let arch = "x86_64";

    writeln!(DebugOutHandle, "\n\nHello {} world!\n\n", arch).unwrap();

    let bootinfo: &'static seL4_BootInfo = unsafe { &*sel4_start::BOOTINFO };
    let num_nodes = bootinfo.numNodes; // Pull out a reference to resolve packed-struct misalignment risk
    writeln!(
        DebugOutHandle,
        "Thing from bootinfo: numNodes={}",
        num_nodes
    )
    .unwrap();

    let suspend_error = unsafe { seL4_TCB_Suspend(seL4_CapInitThreadTCB as usize) };
    if suspend_error != 0 {
        writeln!(
            DebugOutHandle,
            "Error suspending root task thread: {}",
            suspend_error
        )
        .unwrap();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sel4_start::debug_panic_handler(&info)
}