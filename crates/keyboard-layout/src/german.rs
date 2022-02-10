use super::KeyboardLayout;

pub const GERMAN_QWERTZ_LAYOUT: KeyboardLayout = [
    0 as char, /* escape */
    0 as char, /* ^ */
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'ß', '\'', 0 as char, /* backspace */
    0 as char, /* Tab */
    'q', 'w', 'e', 'r', 't', 'z', 'u', 'i', 'o', 'p', 'ü', '+', 0 as char, 0 as char, 'a', 's',
    'd', 'f', 'g', 'h', 'j', 'k', 'l', 'ö', 'ä', '#', 0 as char, '<', 'y', 'x', 'c', 'v', 'b', 'n',
    'm', ',', '.', '_', 0 as char, '*', 0 as char, ' ',
];
