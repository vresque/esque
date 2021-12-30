#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(rustc_private)]
#![feature(abi_x86_interrupt)]
#![allow(unstable_features)]
#![feature(adt_const_params)]
#![feature(alloc_error_handler)]
#![allow(unused_unsafe)]
#![feature(int_log)]

extern crate alloc;
pub use alloc::*;

mod framebuffer;
mod gdt;
mod init;
mod memory;
mod panic;
use alloc::{string::String, vec::Vec};
use bks::{Config, Handover};
mod config;
mod drivers;
mod heap;
mod initramfs;
mod interrupts;
mod iobus;
mod pic;
mod scheduler;
mod userspace;
use config::config;
use initramfs::INITRAMFS;
use scheduler::pit::sleep;
use userspace::{launchpad::Launchpad, pid::Pid};

use crate::scheduler::pit::TIME_SINCE_BOOT;

const HEAP_ADDRESS: u64 = 0x0000100000;
const HEAP_LENGTH: usize = 0x1000;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    init::config::init_config(&mut handover);
    init::gdt::init_gdt(&mut handover);
    init::common::init_common(&mut handover);
    init::memory::init_paging(&mut handover);
    init::interrupts::init_interrupts(&mut handover);
    init::pic::init_pic(&mut handover);
    init::pit::init_pit(&mut handover);
    init::memory::map_memory(&mut handover);
    init::memory::init_heap(&mut handover);
    drivers::init_fallback_drivers(&mut handover);
    //init::security::setup_security(&mut handover);
    //init::ipc::init_ipc(&mut handover);
    initramfs::load_initramfs(&mut handover);

    //init::executive::launch_executive(&mut handover);
    unsafe {
        Launchpad::new(*INITRAMFS.lock().assume_init_mut(), "initramfs/.INITRAMFS")
            .with_pid(Pid::force_new(1))
            .launch();
    }
    // Consumes Handover
    init::userspace::init_userspace(handover);
    loop {}
}
