#[inline]
pub const fn align_up(addr: u64, align: u64) -> u64 {
    assert!(
        align.is_power_of_two(),
        "Addresses may only be aligned to powers of two"
    );
    let mask = align - 1;
    if addr & mask == 0 {
        // If this is true, the address was already aligned
        addr
    } else {
        (addr | mask) + 1 /* Re-Add Mask */
    }
}

#[inline]
pub const fn align_down(addr: u64, align: u64) -> u64 {
    assert!(
        align.is_power_of_two(),
        "Addresses may only be aligned to powers of two"
    );
    let mask = addr - 1;
    addr & !mask
}
