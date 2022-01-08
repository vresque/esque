#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(rustc_private)]
#![feature(abi_x86_interrupt)]
#![allow(unstable_features)]
#![feature(alloc_error_handler)]
#![allow(unused_unsafe)]
#![feature(int_log)]
#![allow(dead_code)]
// Functions and structs may not be used immediately, but may be added in case it will ever be needed
#![deny(unreachable_patterns)] // May lead to certain code not being reached due to bad code

extern crate alloc;
pub use alloc::*;

mod framebuffer;
mod gdt;
mod init;
mod memory;
mod panic;
use bks::Handover;
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
use userspace::{pid::Pid, process::Process};

use crate::heap::{free, malloc, malloc_ptr};

const HEAP_ADDRESS: u64 = 0x0000900000;
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
    let vec = vec![23, 12, 342, 234];
    success!("{:?}", vec);
    loop {}
    //init::ipc::init_ipc(&mut handover);
    initramfs::load_initramfs(&mut handover);
    initramfs::load_kernel_modules_in_initramfs(&mut handover);
    let process = Process::new(Pid::random(), 0, 0, false);

    loop {}
}
