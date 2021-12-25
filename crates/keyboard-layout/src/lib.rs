#![no_std]

mod german;
mod qwerty;
pub mod translator;
use german::GERMAN_QWERTZ_LAYOUT;
use qwerty::QWERTY_LAYOUT;

use bks::KEYBOARD_LAYOUTS_SUPPORTED_NUM;

pub use translator::*;
const ASCII_CHAR_NUM: usize = 58;

enumtastic::const_enum! {
    // ASCII Table:
    pub enum Modifier: u8 => {
        LeftShift = 0x2A,
        RightShift = 0x36,
        Enter = 0x1C,
        BackSpace = 0x0E,
        Spacebar = 0x39,
    }

    impl {}
}

pub const RELEASED_COUNTERPART: u8 = 0x80;

type KeyboardLayout = [char; ASCII_CHAR_NUM];
pub const KEYBOARD_LAYOUTS: [KeyboardLayout; KEYBOARD_LAYOUTS_SUPPORTED_NUM] =
    [QWERTY_LAYOUT, GERMAN_QWERTZ_LAYOUT];
