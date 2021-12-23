#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(asm)]
#![feature(rustc_private)]
#![feature(global_asm)]

mod framebuffer;
mod gdt;
mod init;
mod memory;
mod panic;
use bks::Handover;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    init::gdt::init_gdt(&mut handover);
    init::interrupts::init_interrupts(&mut handover);
    init::common::init_common(&mut handover);
    //init::memory::init_memory(&mut handover);
    loop {}
}
