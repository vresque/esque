use core::alloc::GlobalAlloc;

use crate::heap::GLOBAL_HEAP;

#[global_allocator]
static ALLOCATOR: HeapAllocator = HeapAllocator;

#[derive(Copy, Clone, Default, Debug)]
pub struct HeapAllocator;

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = GLOBAL_HEAP.lock().assume_init_mut().malloc(layout.size());
        ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        GLOBAL_HEAP.lock().assume_init_mut().free(ptr as u64);
    }
}

#[alloc_error_handler]
fn out_of_memory(layout: ::core::alloc::Layout) -> ! {
    panic!(
        "An Out of Memory-Error occurred while trying to allocate with the following layout: {:#?}",
        layout
    )
}
