#![no_std]

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[macro_export]
macro_rules! uasm {
    ($($arg:tt)*) => {
        (unsafe { core::arch::asm!($($arg)*) })
    }
}

#[macro_export]
macro_rules! single_instruction {
    ($name:ident -> $instr:literal) => {
        pub fn $name() {
            crate::uasm!($instr)
        }
    };
}
