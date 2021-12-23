#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(asm)]
#![feature(rustc_private)]
#![feature(global_asm)]
#![feature(abi_x86_interrupt)]
#![feature(adt_const_params)]

mod framebuffer;
mod gdt;
mod init;
mod memory;
mod panic;
use bks::Handover;
mod interrupts;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    init::gdt::init_gdt(&mut handover);
    init::common::init_common(&mut handover);
    init::interrupts::init_interrupts(&mut handover);
    //init::memory::init_memory(&mut handover);
    loop {}
}
