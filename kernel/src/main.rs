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
use esys::{
    ipc::{message::ptr::MessagePointer2, MessageContent},
    process::Process,
};
use userspace::pid::{KernelPid, Pid};
mod ipc;

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
    initramfs::load_initramfs(&mut handover);
    debug!("Still alive");
    initramfs::load_kernel_modules_in_initramfs(&mut handover);
    let process = Process::new(Pid::force_new(1), 0, 0, false);

    let message = esys::ipc::Message::new(
        process,
        process,
        2,
        MessageContent {
            ptr2: MessagePointer2::new(0xffff, 0xff2, [0u64; 5]),
        },
    );
    debug!("{:#?}", message.content.ptr2);

    loop {}
}
