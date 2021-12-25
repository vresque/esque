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
mod interrupts;
mod iobus;
mod pic;
mod syscall;
use config::config;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    init::config::init_config(&mut handover);
    init::gdt::init_gdt(&mut handover);
    init::common::init_common(&mut handover);
    init::memory::init_paging(&mut handover);
    init::interrupts::init_interrupts(&mut handover);
    init::pic::init_pic(&mut handover);
    //init::memory::map_memory(&mut handover); FIXME: This does not work
    drivers::init_fallback_drivers(&mut handover);

    // Consumes Handover
    init::userspace::init_userspace(handover);
    //Launchpad::new(INITRAMFS, "init").launch();
    loop {}
}
