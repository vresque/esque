use crate::arch::{HEAP_ADDRESS, HEAP_LENGTH};
use crate::heap::Heap;
use crate::info;
use bks::Handover;

pub fn init_heap() {
    info!("Initializing Heap!");
    unsafe {
        crate::heap::GLOBAL_HEAP
            .lock()
            .write(Heap::new(HEAP_ADDRESS, HEAP_LENGTH));
    }
}
