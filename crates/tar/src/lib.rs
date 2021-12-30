#![no_std]

use arrayvec::ArrayString;
extern crate alloc;
pub mod header;
pub mod tar;
pub mod types;

pub fn as_array_string<'dta, const N: usize>(slice: &'dta [u8]) -> ArrayString<N> {
    let mut string = arrayvec::ArrayString::<N>::new();
    slice
        .iter()
        .filter(|x| **x != 0)
        .for_each(|c| string.push(*c as char));
    string
}

// https://www.gnu.org/software/tar/manual/html_node/Standard.html
