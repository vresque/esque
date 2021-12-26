use core::mem::MaybeUninit;

use bks::{EfiMemoryDescriptor, MemoryType, PAGE_SIZE};

use crate::kprintln;

use crate::memory::bitmap::Bitmap;
use spin::Mutex;

pub static PAGE_FRAME_ALLOCATOR: Mutex<MaybeUninit<PageFrameAllocator>> =
    Mutex::new(MaybeUninit::uninit());
static mut IS_PAGE_FRAME_ALLOCATOR_INITIALIZED: bool = false;
pub static mut REJECTS: u64 = 0;

pub struct PageFrameAllocator<'a> {
    base: u64,
    map: &'a mut [EfiMemoryDescriptor],
    entries: usize,
    entry_size: usize,
    size: usize,
    bitmap: Bitmap,
    free: i64, // FIXME: During Initialization, the value may drop below zero
    reserved: i64,
    used: i64,
    last_bmap_index: u64,
}

impl<'a> PageFrameAllocator<'a> {
    pub fn new(base: *mut u8, entries: usize, size: usize, entry_size: usize) -> Self {
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
            entry_size,
            bitmap,
            free: 0,
            reserved: 0,
            used: 0,
            last_bmap_index: 0,
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
        kprintln!("{}mb", mem_sz / 1024 / 1024 / 1024);
        self.free = mem_sz as i64;
        // One for each page
        let bitmap_size = mem_sz / PAGE_SIZE / 8 + 1;
        kprintln!("{}", bitmap_size);

        // Initialize Bitmap
        self.initialize_bitmap(bitmap_size as usize, current_largest_free_segment);
        self.lock_pages(
            &self.bitmap as *const Bitmap as *const u64 as u64,
            self.bitmap.size / PAGE_SIZE as usize + 1,
        );

        for i in 0..self.entries {
            let ent = self.map[i];
            if ent.ty != MemoryType::ConventialMemory {
                self.reserve_pages(ent.phys_base, ent.page_count as usize);
            }
        }
        kprintln!("{}", self.free);
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
        if self.bitmap.set(idx as usize, false) {
            self.free += PAGE_SIZE as i64;
            self.used -= PAGE_SIZE as i64;
            if self.last_bmap_index > idx {
                self.last_bmap_index = idx
            }
        }
    }

    pub fn free_pages(&mut self, addr: u64, count: usize) {
        for i in 0..count {
            self.free_page((addr + (i as u64 * PAGE_SIZE)) as u64)
        }
    }

    pub fn lock_page(&mut self, addr: u64) {
        let idx = addr / PAGE_SIZE;

        // Already locked
        if self.bitmap[idx as usize] == true {
            return;
        }

        if self.bitmap.set(idx as usize, true) {
            self.free -= PAGE_SIZE as i64;
            self.used += PAGE_SIZE as i64;
        }
    }

    pub fn lock_pages(&mut self, addr: u64, count: usize) {
        for i in 0..count {
            self.lock_page((addr + (i as u64 * PAGE_SIZE)) as u64);
        }
    }

    fn reserve_page(&mut self, addr: u64) {
        let idx = addr / PAGE_SIZE;
        // Already free
        if self.bitmap[idx as usize] == true {
            return;
        }
        if self.bitmap.set(idx as usize, true) {
            self.free -= PAGE_SIZE as i64;
            self.reserved += PAGE_SIZE as i64;
        }
    }

    pub fn reserve_pages(&mut self, addr: u64, count: usize) {
        for i in 0..count {
            self.reserve_page((addr + (i as u64 * PAGE_SIZE)) as u64)
        }
    }

    fn release_page(&mut self, addr: u64) {
        let idx = addr / PAGE_SIZE;
        // Already allocated
        if self.bitmap[idx as usize] == false {
            return;
        }
        if self.bitmap.set(idx as usize, true) {
            self.free += PAGE_SIZE as i64;
            self.reserved -= PAGE_SIZE as i64;
            if self.last_bmap_index > idx {
                self.last_bmap_index = idx;
            }
        }
    }

    pub fn release_pages(&mut self, addr: u64, count: usize) {
        for i in 0..count {
            self.release_page((addr + (i as u64 * PAGE_SIZE)) as u64)
        }
    }

    // TODO: Optimize
    pub fn request_page(&mut self) -> u64 {
        while self.last_bmap_index < (self.bitmap.size as u64 * 8 as u64) {
            if self.bitmap[self.last_bmap_index as usize] == false {
                self.lock_page(self.last_bmap_index * 4096);
                return self.last_bmap_index * PAGE_SIZE;
            }
            self.last_bmap_index += 1;
        }
        unsafe {
            REJECTS += 1;
        }
        return 0;
    }

    pub fn total_memory(&self) -> u64 {
        kprintln!("LENEN: {}", self.map.len());
        unsafe {
            static mut MEM_SZ_BYTES: u64 = 0;
            // If calculated before, just return it
            if MEM_SZ_BYTES > 0 {
                return MEM_SZ_BYTES;
            }

            for i in self.map.iter() {
                MEM_SZ_BYTES += i.page_count * PAGE_SIZE;
                if i.ty != MemoryType::EmptyTemporaryMemory {
                    kprintln!("{:#?}", i);
                }
            }

            MEM_SZ_BYTES
        }
    }

    pub fn get_free_memory(&self) -> i64 {
        self.free
    }

    pub fn get_used_memory(&self) -> i64 {
        self.used
    }

    pub fn get_reserved_memory(&self) -> i64 {
        self.reserved
    }
}

pub fn request_page<'retval, T>() -> &'retval mut T {
    unsafe {
        &mut *(PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page() as *mut u64 as *mut T)
    }
}
