use super::KeyboardLayout;

//https://github.com/Absurdponcho/PonchoOS/blob/Episode-13-KB-Scancodes/kernel/src/userinput/kbScancodeTranslation.cpp
pub const QWERTY_LAYOUT: KeyboardLayout = [
    0 as char, 0 as char, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 0 as char,
    0 as char, 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', 0 as char, 0 as char,
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`', 0 as char, '\\', 'z', 'x', 'c',
    'v', 'b', 'n', 'm', ',', '.', '/', 0 as char, '*', 0 as char, ' ',
];
