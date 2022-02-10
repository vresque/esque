//! This entire file is largely based on the examples in https://wiki.osdev.org/PIC#Programming_the_PIC_chips

use core::arch::asm;
pub mod msr;

/// # In Bus
/// Reads a byte from the given port
#[inline(always)]
pub fn inb(port: u16) -> u8 {
    let mut retval: u8;
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") retval, options(preserves_flags, nomem, nostack));
    }
    retval
}
/// # Out Bus
/// Writes the value to the port
#[inline(always)]
pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(preserves_flags, nomem, nostack));
    }
}
/// # IO Wait
/// Waits to let devices catch up
#[inline(always)]
pub fn io_wait() {
    inb(0x80);
}

#[inline]
pub fn io_wait_for(cycles: usize) {
    for _ in 0..cycles {
        io_wait();
    }
}

/// # Out Low
/// Low Level Port Output with a u32 value
#[inline(always)]
pub fn outl(port: u16, value: u32) {
    unsafe {
        asm!(
                "out dx, eax",
                in("dx") port,
                in("eax") value,
                options(preserves_flags, nomem, nostack)
        )
    }
}

#[inline(always)]
pub fn inl(port: u16) -> u32 {
    let mut retval: u32;
    unsafe {
        asm!("in eax, dx", in("dx") port, out("eax") retval, options(preserves_flags, nomem, nostack));
    };
    retval
}
