#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bks::Handover;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    handover.framebuffer().clear_bcolor(0xff);
    22
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
