use bks::KEYBOARD_LAYOUTS_SUPPORTED_NUM;

use crate::{ASCII_CHAR_NUM, KEYBOARD_LAYOUTS};

pub fn translate_from_u8(scancode: u8, uppercase: bool, layout: usize) -> char {
    if scancode > ASCII_CHAR_NUM as u8 {
        return 0 as char;
    }
    if layout > KEYBOARD_LAYOUTS_SUPPORTED_NUM {
        return 0 as char;
    }
    return KEYBOARD_LAYOUTS[layout][scancode as usize];
}
