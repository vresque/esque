#![no_std]

mod german;
mod qwerty;
pub mod translator;
use german::GERMAN_QWERTZ_LAYOUT;
use qwerty::QWERTY_LAYOUT;
pub use translator::*;
const ASCII_CHAR_NUM: usize = 58;
type KeyboardLayout = [char; ASCII_CHAR_NUM];
use bks::KEYBOARD_LAYOUTS_SUPPORTED_NUM;
pub const KEYBOARD_LAYOUTS: [KeyboardLayout; KEYBOARD_LAYOUTS_SUPPORTED_NUM] =
    [QWERTY_LAYOUT, GERMAN_QWERTZ_LAYOUT];
