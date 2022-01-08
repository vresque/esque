use core::mem::{size_of, MaybeUninit};

use bks::PAGE_SIZE;
use spin::Mutex;
use unique::Unique;

use crate::{
    debug, kprintln,
    memory::paging::{
        page_frame_allocator::PAGE_FRAME_ALLOCATOR, page_table_manager::GLOBAL_PAGE_TABLE_MANAGER,
    },
};
pub static GLOBAL_HEAP: Mutex<MaybeUninit<Heap>> = Mutex::new(MaybeUninit::uninit());

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeapSegmentHeader {
    len: usize,
    next: Option<Unique<HeapSegmentHeader>>,
    last: Option<Unique<HeapSegmentHeader>>,
    free: bool,
}

unsafe impl Send for HeapSegmentHeader {}

impl HeapSegmentHeader {
    pub fn new(
        len: usize,
        next: Option<Unique<HeapSegmentHeader>>,
        last: Option<Unique<HeapSegmentHeader>>,
        free: bool,
    ) -> Self {
        Self {
            len,
            next,
            last,
            free,
        }
    }

    pub unsafe fn from_addr<'header>(addr: u64) -> &'header mut Self {
        let me = &mut *(addr as *mut u64 as *mut Self);
        me
    }

    /// # Genesis
    /// Produces the first header of the heap
    pub unsafe fn genesis<'header>(address: u64, length: usize) -> &'header mut Self {
        let mut me = &mut *(address as *mut u64 as *mut HeapSegmentHeader);
        me.len = length - core::mem::size_of::<HeapSegmentHeader>();
        me.next = None;
        me.last = None;
        me.free = true;
        me
    }
    /// # Combine With Next
    /// Combines the current header with the next segment header extending it
    pub fn combine_with_next<'heap_last_header_lt>(
        &'heap_last_header_lt mut self,
        // This is borrowed to avoid me having to spend 15s changing the current API
        heap_last_hdr_borrow: &'heap_last_header_lt mut HeapSegmentHeader,
    ) {
        let mut heap_last_hdr = Unique::new_borrowed_mut(heap_last_hdr_borrow).unwrap();
        if let Some(mut next) = self.next {
            unsafe {
                if next.as_mut().free == false {
                    return;
                }
                if next == heap_last_hdr {
                    heap_last_hdr = Unique::new_borrowed_mut(self).unwrap();
                }

                if let Some(mut next_next) = next.as_mut().next {
                    next_next.as_mut().last = Some(Unique::new(self).unwrap());
                }

                self.len += next.as_mut().len + size_of::<HeapSegmentHeader>();
                self.next = next.as_mut().next;
            }
        } else {
            return;
        }
    }

    /// # Combine With Last
    /// Combines the current header with the last segment header, extending it
    pub fn combine_with_last(&mut self, heap_last_hdr: &mut HeapSegmentHeader) {
        if let Some(mut last) = self.last {
            unsafe {
                if last.as_mut().free == true {
                    last.as_mut().combine_with_next(heap_last_hdr);
                }
            }
        } else {
        }
    }

    /// # Split
    /// Splits the Segment Header into two headers, ending the first after the given length
    /// ## Parameters
    /// - `after: usize` = The Size after which the header is split
    pub fn split<'header>(
        &mut self,
        len_after_split: usize,
    ) -> Option<&'header mut HeapSegmentHeader> {
        // Minimum Header Size
        if len_after_split < 0x10 {
            return None;
        }
        // This may be less than zero
        let new_segment_length: isize =
            self.len as isize - len_after_split as isize - size_of::<HeapSegmentHeader>() as isize;
        // If it is too smal OR if it is below zero, there is no point splitting it
        if new_segment_length < 0x10 {
            return None;
        }

        let new_split_header = unsafe {
            HeapSegmentHeader::from_addr(
                self.address() + new_segment_length as u64 + size_of::<HeapSegmentHeader>() as u64,
            )
        };

        // What happens here might be hard to understand, it is illustrated below
        // Before the change:
        // |************| |************|
        // | Header A   | | Header B   |
        // | Next: B    | | Next: None |
        // | Last: None | | Last: A    |
        // |************| |************|
        //
        // After the change
        // |************| |************| |************|
        // | Header A   | | Header New | | Header B   |
        // | Next: New  | | Next: B    | | Next: None |
        // | Last: None | | Last: A    | | Last: New  |
        // |************| |************| |************|
        //                 ^^^^^^^^^^^^
        //                  I am new

        // The Last Header of the Next header is the new header
        if let Some(mut hdr) = self.next {
            unsafe {
                hdr.as_mut().last = Some(Unique::new(new_split_header as *mut _).unwrap());
            }
        }

        // Header New's next is our current next (Header B)
        new_split_header.next = self.next;

        // Our own Next is the new header (Header A's next is Header New now)
        self.next = Some(Unique::new(new_split_header as *mut _).unwrap());

        // The new header's last are we (Header New's last is Header A)
        new_split_header.last =
            Some(Unique::new(self.address() as *mut u64 as *mut HeapSegmentHeader).unwrap());

        // Header New's length is the length calculated before
        new_split_header.len = new_segment_length as usize; // If it was negative, we returned

        // We split it, if it was free before, it is now
        new_split_header.free = self.free;

        // Passed in via param
        self.len = len_after_split;
        Some(new_split_header)
    }

    pub fn as_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }

    pub fn address(&mut self) -> u64 {
        unsafe { self as *mut Self as *mut u64 as u64 }
    }
}

pub struct Heap<'header> {
    // The Last Heap Header
    last_header: &'header mut HeapSegmentHeader,
    heap_start: u64,
    heap_end: u64,
    page_count: usize,
}

impl<'header> Heap<'header> {
    pub unsafe fn new(heap_address: u64, page_count: usize) -> Self {
        for i in (0..page_count).step_by(PAGE_SIZE as usize) {
            let page = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page();
            GLOBAL_PAGE_TABLE_MANAGER
                .lock()
                .assume_init_mut()
                .map_memory(i as u64, page);
        }

        let heap_len_in_bytes = page_count * PAGE_SIZE as usize;

        let heap = Self {
            last_header: HeapSegmentHeader::genesis(heap_address, heap_len_in_bytes),
            heap_start: heap_address,
            heap_end: (heap_address + heap_len_in_bytes as u64),
            page_count: page_count,
        };

        heap
    }

    pub unsafe fn malloc(&mut self, size: usize) -> u64 {
        let rounded_size = if size % 0x10 > 0 {
            // Round up the size to next u64 number
            // Not a multiple of 128. DISCUSS: Should this be 0x08 (u64)
            (size - (size % 0x10)) + 0x10
        } else {
            size
        };

        // The Size is zero, according to the C Standard, we shall do nothing and return NULL
        if rounded_size == 0 {
            return 0;
        }

        // Start at genesis
        let mut current_segment = HeapSegmentHeader::from_addr(self.heap_start);
        loop {
            if current_segment.free {
                if current_segment.len > rounded_size {
                    // We found the perfect block, but it is too big
                    // We split it into one perfect and one imperfect header
                    self.split_header(current_segment, rounded_size);
                    current_segment.free = false;
                    return current_segment.address() + size_of::<HeapSegmentHeader>() as u64;
                }

                // It is a perfect fit
                if current_segment.len == rounded_size {
                    return current_segment.address() + size_of::<HeapSegmentHeader>() as u64;
                }
            }
            // Not free
            if current_segment.next == None {
                // Last Block in Memory (Heap must be extended)
                break;
            }
            current_segment = current_segment.next.unwrap().inner().as_mut();
        }
        // Heap is ending
        // We must extend the heap
        self.expand(rounded_size);
        // SAFETY: This will never recurse twice
        return self.malloc(rounded_size);
    }

    /// # Split Header
    /// Splits the given header *and* sets the own last_header
    pub fn split_header(&mut self, header: &mut HeapSegmentHeader, length_after_split: usize) {
        let new = if let Some(hdr) = header.split(length_after_split) {
            hdr
        } else {
            // If an error occurred, we need not set any variables
            return;
        };

        // If the header was the last header, the new one is now the last header
        if self.last_header == header {
            self.last_header = new;
        }
    }

    pub fn free(&mut self, address: u64) {
        debug!("Freeing {:#x?}", address);
        let header = unsafe {
            HeapSegmentHeader::from_addr(address - size_of::<HeapSegmentHeader>() as u64)
        }; // The Header is always size_of(HeapSegmentHeader) before the actual value
        debug!("{:?}", header);
        header.free = true;
        header.combine_with_last(self.last_header);
        header.combine_with_next(self.last_header);
    }

    fn expand(&mut self, length: usize) {
        let rounded_length = if length % PAGE_SIZE as usize == 0 {
            (length as u64 - (length as u64 % PAGE_SIZE)) + PAGE_SIZE
        } else {
            length as u64
        };

        let page_count = rounded_length / PAGE_SIZE;
        let header: &'header mut HeapSegmentHeader =
            unsafe { HeapSegmentHeader::from_addr(self.heap_end) };

        for i in 0..page_count {
            unsafe {
                let page = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page();
                GLOBAL_PAGE_TABLE_MANAGER
                    .lock()
                    .assume_init_mut()
                    .map_memory(i as u64, page);
                self.heap_end += PAGE_SIZE;
            }
        }

        header.free = true;
        header.last = Some(
            Unique::new(self.last_header.address() as *mut u64 as *mut HeapSegmentHeader).unwrap(),
        );
        self.last_header.next = Some(Unique::new(header).unwrap());
        header.next = None;
        header.len = length - size_of::<HeapSegmentHeader>();
        header.combine_with_last(self.last_header);
        self.last_header = header;
    }
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

pub fn free(addr: u64) {
    unsafe { GLOBAL_HEAP.lock().assume_init_mut().free(addr) }
}
