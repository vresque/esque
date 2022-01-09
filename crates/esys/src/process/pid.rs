use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pid {
    pub id: usize,
}
