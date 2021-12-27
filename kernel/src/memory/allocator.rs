use core::alloc::GlobalAlloc;

use crate::heap::GLOBAL_HEAP;

#[derive(Copy, Clone, Default, Debug)]
pub struct HeapAllocator;

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = GLOBAL_HEAP.lock().assume_init_mut().malloc(layout.size());
        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        GLOBAL_HEAP
            .lock()
            .assume_init_mut()
            .free(ptr as *mut u64 as u64);
    }
}
