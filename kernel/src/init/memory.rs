use bks::{Handover, PAGE_SIZE};

use crate::kinfo;
use crate::memory::bitmap::Bitmap;
use crate::{
    kprintln,
    memory::pageframeallocator::{PageFrameAllocator, PAGE_FRAME_ALLOCATOR},
};

pub fn init_memory(handover: &mut Handover) {
    kprintln!("Initializing memory...");
    unsafe {
        kprintln!("Initializing Bitmap...");
        PAGE_FRAME_ALLOCATOR.lock().write(PageFrameAllocator::new(
            handover.raw_memory_map() as *mut u8,
            handover.mmap_entries,
            handover.mmap_size,
        ));

        PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .read_memory_map();

        let reserved = PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .get_reserved_memory();
        let free = PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .get_free_memory();
        let used = PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .get_used_memory();

        kprintln!(
            "Reserved Memory: {}kb, Free Memory: {}kb, Used Memory: {}kb",
            reserved / 1024,
            free / 1024,
            used / 1024
        )
    };
}
