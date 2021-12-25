use keyboard_layout::translator;

use crate::framebuffer::FRAMEBUFFER_GUARD;
use crate::{config, kprint};
use crate::{
    interrupts::interrupt_frame::InterruptFrame,
    iobus::inb,
    kprintln,
    pic::{end_main_pic, PicPort},
};
use core::fmt::Write;

pub extern "x86-interrupt" fn ps2_keyboard_int_handler(a: InterruptFrame) {
    // Get Keyboard Scancode
    let scancode = inb(PicPort::Ps2KeyboardScancodePort);
    handle_keyboard(scancode);
    end_main_pic();
}

pub fn handle_keyboard(scancode: u8) {
    let ascii = translator::translate_from_u8(scancode, false, config().layout as usize);
    // NULLs cannot be displayed
    if ascii != 0 as char {
        unsafe {
            FRAMEBUFFER_GUARD.lock().assume_init_mut().write_char(ascii);
        }
    }
}
