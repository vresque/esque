#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]
#![feature(format_args_nl)]

use core::{
    ops::{Shr, Sub},
    panic::PanicInfo,
};
mod init;
mod log;
use bks::Handover;
use init::common::init_common;

static mut COUNTER: u32 = 0;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    handover.framebuffer().clear_bcolor(0x0);
    init_common(&mut handover);
    kprintln!("Hello, (Lol). Here is an int {}", 23);
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
