use core::mem::MaybeUninit;

use bks::{EfiMemoryDescriptor, MemoryType, PAGE_SIZE};

use crate::kprintln;

use super::bitmap::Bitmap;
use spin::Mutex;

pub static PAGE_FRAME_ALLOCATOR: Mutex<MaybeUninit<PageFrameAllocator>> =
    Mutex::new(MaybeUninit::uninit());
static mut IS_PAGE_FRAME_ALLOCATOR_INITIALIZED: bool = false;

pub struct PageFrameAllocator<'a> {
    base: u64,
    map: &'a mut [EfiMemoryDescriptor],
    entries: usize,
    size: usize,
    bitmap: Bitmap,
    free: u64,
    reserved: u64,
    used: u64,
}

impl<'a> PageFrameAllocator<'a> {
    pub fn new(base: *mut u8, entries: usize, size: usize) -> Self {
        if unsafe { IS_PAGE_FRAME_ALLOCATOR_INITIALIZED } {
            panic!("Tried to create a PageFrameAllocator twice in the same session.")
        }

        let map =
            unsafe { core::slice::from_raw_parts_mut(base as *mut EfiMemoryDescriptor, entries) };
        let bitmap = Bitmap::new(core::ptr::null_mut(), 0);

        unsafe {
            IS_PAGE_FRAME_ALLOCATOR_INITIALIZED = true;
        }
        Self {
            base: base as u64,
            map,
            entries,
            size,
            bitmap,
            free: 0,
            reserved: 0,
            used: 0,
        }
    }

    pub fn read_memory_map(&mut self) {
        let mut current_largest_free_segment: u64 = 0;
        let mut current_largest_free_segment_size = 0;
        for ent in self.map.iter() {
            match ent.ty {
                MemoryType::ConventialMemory => {
                    if ent.page_count * PAGE_SIZE > current_largest_free_segment_size {
                        current_largest_free_segment = ent.phys_base;
                        current_largest_free_segment_size = ent.page_count * PAGE_SIZE;
                    }
                }
                _ => {}
            }
        }

        let mem_sz = self.total_memory();
        kprintln!("{}", mem_sz);
        self.free = mem_sz;
        // One for each page
        let bitmap_size = mem_sz / PAGE_SIZE / 8 + 1;
        kprintln!("{}", bitmap_size);

        // Initialize Bitmap
        self.initialize_bitmap(bitmap_size as usize, current_largest_free_segment);

        // Lock ourself
        let bmap_addr = &mut self.bitmap as *mut Bitmap as u64;
        self.lock_pages(bmap_addr, self.bitmap.size / 4096 + 1);
        self.lock_pages(self.bitmap.base, self.bitmap.size / 4096 + 1);

        for i in 0..self.entries {
            let ent = self.map[i];
            if ent.ty != MemoryType::ConventialMemory {
                self.reserve_pages(ent.phys_base, ent.page_count as usize);
            }
        }
    }

    fn initialize_bitmap(&mut self, bmp_size: usize, addr: u64) {
        self.bitmap = Bitmap::new(addr as *mut u8, bmp_size);
        for i in 0..bmp_size {
            // Reset Bitmap
            unsafe {
                *((self.bitmap.base as *mut u8).add(i)) = 0;
            }
        }
    }

    pub fn free_page(&mut self, addr: u64) {
        let idx = addr / PAGE_SIZE;
        // Already free
        if self.bitmap[idx as usize] == false {
            return;
        }
        self.bitmap.set(idx as usize, false);
        self.free += PAGE_SIZE;
        self.used -= PAGE_SIZE;
    }

    pub fn free_pages(&mut self, addr: u64, count: usize) {
        for i in 0..count {
            self.free_page((addr + (i as u64 * PAGE_SIZE)) as u64)
        }
    }

    pub fn lock_page(&mut self, addr: u64) {
        let idx = addr / PAGE_SIZE;
        // Already free
        if self.bitmap[idx as usize] == true {
            return;
        }
        self.bitmap.set(idx as usize, true);
        self.free -= PAGE_SIZE;
        self.used += PAGE_SIZE;
    }

    pub fn lock_pages(&mut self, addr: u64, count: usize) {
        for i in 0..count {
            self.lock_page((addr + (i as u64 * PAGE_SIZE)) as u64)
        }
    }

    fn reserve_page(&mut self, addr: u64) {
        let idx = addr / PAGE_SIZE;
        // Already free
        if self.bitmap[idx as usize] == true {
            return;
        }
        self.bitmap.set(idx as usize, true);
        self.free -= PAGE_SIZE;
        self.reserved += PAGE_SIZE;
    }

    pub fn reserve_pages(&mut self, addr: u64, count: usize) {
        for i in 0..count {
            self.reserve_page((addr + (i as u64 * PAGE_SIZE)) as u64)
        }
    }

    fn release_page(&mut self, addr: u64) {
        let idx = addr / PAGE_SIZE;
        // Already free
        if self.bitmap[idx as usize] == true {
            return;
        }
        self.bitmap.set(idx as usize, true);
        self.free += PAGE_SIZE;
        self.reserved -= PAGE_SIZE;
    }

    pub fn release_pages(&mut self, addr: u64, count: usize) {
        for i in 0..count {
            self.release_page((addr + (i as u64 * PAGE_SIZE)) as u64)
        }
    }

    pub fn total_memory(&self) -> u64 {
        static mut mem_sz_bytes: u64 = 0;
        // If calculated before, just return it
        if unsafe { mem_sz_bytes } > 0 {
            unsafe {
                return mem_sz_bytes;
            }
        }
        let mut sum_of_mem_sizes = 0;
        for i in self.map.iter() {
            sum_of_mem_sizes += i.page_count;
        }

        unsafe {
            mem_sz_bytes = sum_of_mem_sizes;
            mem_sz_bytes
        }
    }

    pub fn get_free_memory(&self) -> u64 {
        self.free
    }

    pub fn get_used_memory(&self) -> u64 {
        self.used
    }

    pub fn get_reserved_memory(&self) -> u64 {
        self.reserved
    }
}
