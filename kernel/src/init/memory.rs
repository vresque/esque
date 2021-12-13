use bks::{Handover, PAGE_SIZE};

use crate::kinfo;
use crate::memory::bitmap::Bitmap;
use crate::{
    kprintln,
    memory::pageframeallocator::{PageFrameAllocator, PAGE_FRAME_ALLOCATOR},
};

#[no_mangle]
static _KERNEL_START: u64 = 0;

#[no_mangle]
static _KERNEL_END: u64 = 0;

pub fn init_memory(handover: &mut Handover) {
    kprintln!("Initializing memory...");
    unsafe {
        kprintln!("Initializing PageFrameAllocator...");
        PAGE_FRAME_ALLOCATOR.lock().write(PageFrameAllocator::new(
            handover.raw_memory_map() as *mut u8,
            handover.mmap_entries,
            handover.mmap_size,
        ));

        kprintln!("Intialized PageFrameAllocator");

        PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .read_memory_map();

        kprintln!("Locking Kernel Pages...");
        let kernel_size =
            (&_KERNEL_END as *const u64 as u64 - &_KERNEL_START as *const u64 as u64) as u64;

        // Lock Kernel
        let kernel_pages = kernel_size / PAGE_SIZE + 1;
        kprintln!("{}", kernel_pages);
        PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .lock_pages(&_KERNEL_START as *const u64 as u64, kernel_pages as usize);

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
            "Reserved Memory: {}mb, Free Memory: {}mb, Used Memory: {}mb",
            reserved / 1024 / 1024,
            free / 1024 / 1024,
            used / 1024 / 1024,
        );
        for i in 0..20 {
            let addr = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page();
            kprintln!("{:#x?}", addr);
        }
    };
}
