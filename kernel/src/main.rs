#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(asm)]

mod framebuffer;
mod init;
mod panic;
use bks::Handover;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    init::common::init_common(&mut handover);
    init::memory::init_memory(&mut handover);
    kinfo!("Lol");
    kscopedcolorchange!(bg: framebuffer::Color::Red, fg: framebuffer::Color::White => {
        { kprintln!("Lol!"); }
    });
    kprintln!("Scope ended here!");
    loop {}
}
