#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bks::Handover;

unsafe fn draw_char(
    mut handover: &mut Handover,
    colour: u32,
    chr: char,
    x_offset: u32,
    y_offset: u32,
) {
    const BPP: usize = 4;
    let offset = (chr as u8 - 32) as usize * 16;
    for y in 0..16 {
        for x in 0..8 {
            let cur_x = x_offset as usize + (8 - x);
            let cur_y = y_offset as usize + y;

            let ptr = ((handover.framebuffer().raw_buffer() as usize)
                + (cur_x * (BPP / 8) as usize + cur_y * handover.framebuffer().stride as usize)
                    as usize) as *mut u32;

            if handover.font().buffer()[y + offset as usize] >> x & 1 == 1 {
                *ptr = colour;
            }
        }
    }
}

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    unsafe {
        draw_char(&mut handover, 0xffffffff, 'G', 20, 20);
    };

    handover.font().header().charsize() as u32
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
