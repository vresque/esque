#![no_std]
#![no_main]

use esque::argc;
extern crate esque;

#[no_mangle]
pub fn main() -> u64 {
    let argc = argc();
    return argc * argc + 1;
}
