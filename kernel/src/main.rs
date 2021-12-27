#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(rustc_private)]
#![feature(abi_x86_interrupt)]
#![allow(unstable_features)]
#![feature(adt_const_params)]
mod framebuffer;
mod gdt;
mod init;
mod memory;
mod panic;
use bks::{Config, Handover};
mod config;
mod drivers;
mod heap;
mod interrupts;
mod iobus;
mod pic;
use config::config;

const HEAP_ADDRESS: u64 = 0x0000100000;
const HEAP_LENGTH: usize = 0x1000;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    init::config::init_config(&mut handover);
    init::gdt::init_gdt(&mut handover);
    init::common::init_common(&mut handover);
    init::memory::init_paging(&mut handover);
    init::memory::map_memory(&mut handover);
    init::interrupts::init_interrupts(&mut handover);
    init::memory::init_heap(&mut handover);
    init::pic::init_pic(&mut handover);
    drivers::init_fallback_drivers(&mut handover);

    //Launchpad::new(INITRAMFS, "initfs").launch();
    // Consumes Handover
    init::userspace::init_userspace(handover);
    loop {}
}
