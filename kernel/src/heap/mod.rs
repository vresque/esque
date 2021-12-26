use core::{mem::MaybeUninit, ptr::NonNull};

use bks::PAGE_SIZE;
use spin::Mutex;

use crate::{
    memory::paging::{
        page_frame_allocator::request_page, page_table_manager::GLOBAL_PAGE_TABLE_MANAGER,
    },
    HEAP_LENGTH,
};
pub static GLOBAL_HEAP: Mutex<MaybeUninit<Heap>> = Mutex::new(MaybeUninit::uninit());

#[derive(Clone, Copy, Debug)]
pub struct HeapSegmentHeader {
    len: usize,
    next: Option<NonNull<HeapSegmentHeader>>,
    last: Option<NonNull<HeapSegmentHeader>>,
    free: bool,
}

unsafe impl Send for HeapSegmentHeader {}

impl HeapSegmentHeader {
    pub fn new(
        len: usize,
        next: Option<NonNull<HeapSegmentHeader>>,
        last: Option<NonNull<HeapSegmentHeader>>,
        free: bool,
    ) -> Self {
        Self {
            len,
            next,
            last,
            free,
        }
    }
    /// # Genesis
    /// Produces the first header of the heap
    pub unsafe fn genesis(address: u64, length: usize) -> Self {
        let mut me = *(address as *mut u64 as *mut HeapSegmentHeader);
        me.len = length - core::mem::size_of::<HeapSegmentHeader>();
        me.next = None;
        me.last = None;
        me.free = true;
        me
    }
    /// # Combine With Next
    /// Combines the current header with the next segment header extending it
    pub fn combine_with_next(&mut self) {}
    /// # Combine With Last
    /// Combines the current header with the last segment header, extending it
    pub fn combine_with_last(&mut self) {}
    /// # Split
    /// Splits the Segment Header into two headers, ending the first after the given length
    /// ## Parameters
    /// - `after: usize` = The Size after which the header is split
    pub fn split(&mut self, after: usize) {}
}

pub struct Heap {
    // The Last Heap Header
    last_header: HeapSegmentHeader,
    heap_start: u64,
    heap_end: u64,
    page_count: usize,
}

impl Heap {
    pub unsafe fn new(heap_address: u64, page_count: usize) -> Self {
        for i in (0..page_count).step_by(PAGE_SIZE as usize) {
            GLOBAL_PAGE_TABLE_MANAGER
                .lock()
                .assume_init_mut()
                .map_memory(i as u64, *request_page());
        }

        let heap_len_in_bytes = page_count * PAGE_SIZE as usize;

        let mut heap = Self {
            last_header: HeapSegmentHeader::genesis(heap_address, heap_len_in_bytes),
            heap_start: heap_address,
            heap_end: (heap_address + heap_len_in_bytes as u64),
            page_count: page_count,
        };

        heap
    }

    pub fn malloc(&mut self, size: usize) -> u64 {
        0
    }
    pub fn free(&mut self, address: u64) {}
    fn extend(length: usize) {}
}

pub unsafe fn malloc_ptr<T>(size: usize) -> *mut T {
    GLOBAL_HEAP.lock().assume_init_mut().malloc(size) as *mut T
}

pub fn malloc<'ptr_lifetime, T>() -> &'ptr_lifetime T {
    malloc_mut()
}

pub fn malloc_mut<'ptr_lifetime, T>() -> &'ptr_lifetime mut T {
    unsafe { &mut *(malloc_ptr(core::mem::size_of::<T>())) }
}

pub fn free(addr: u64) {}
