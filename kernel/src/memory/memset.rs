pub unsafe fn memset(start: u64, value: u8, count: usize) {
    for i in 0..count {
        *((start as *mut u64).add(i)) = value as u64;
    }
}
