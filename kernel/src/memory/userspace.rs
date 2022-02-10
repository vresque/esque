use spin::Mutex;

use crate::init::memory::{_KERNEL_END, _KERNEL_START};

pub static LAST_VIRT_MEM: Mutex<u64> = Mutex::new((_KERNEL_START + _KERNEL_END) + (10 * 1024));
