#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]

use core::{
    ops::{Shr, Sub},
    panic::PanicInfo,
};
mod init;
mod log;
use bks::Handover;
use init::common::init_common;

static mut COUNTER: u32 = 0;
static mut HANDOVER: Option<Handover> = None;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    handover.framebuffer().clear_bcolor(0x0);
    unsafe { HANDOVER = Some(handover) };
    init_common();

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
