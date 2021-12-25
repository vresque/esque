use keyboard_layout::translator;

use crate::{config, kprint};
use crate::{
    interrupts::interrupt_frame::InterruptFrame,
    iobus::inb,
    kprintln,
    pic::{end_main_pic, PicPort},
};

pub extern "x86-interrupt" fn ps2_keyboard_int_handler(a: InterruptFrame) {
    // Get Keyboard Scancode
    let scancode = inb(PicPort::Ps2KeyboardScancodePort);
    end_main_pic();
    handle_keyboard(scancode);
}

pub fn handle_keyboard(scancode: u8) {
    let ascii = translator::translate_from_u8(scancode, false, config().layout as usize);
    // NULLs cannot be displayed
    if ascii != 0 as char {
        kprint!("{}", ascii);
    }
}
