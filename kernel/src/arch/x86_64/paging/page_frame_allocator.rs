use core::mem::MaybeUninit;
use core::ptr::NonNull;

use bks::{EfiMemoryDescriptor, MemoryType, PAGE_SIZE};

use crate::arch::init::memory::_KERNEL_OFFSET;
use crate::math::is_aligned;
use crate::{debug, kprintln};

use crate::memory::bitmap::Bitmap;
use spin::Mutex;

pub static PAGE_FRAME_ALLOCATOR: Mutex<MaybeUninit<PageFrameAllocator>> =
    Mutex::new(MaybeUninit::uninit());
static mut IS_PAGE_FRAME_ALLOCATOR_INITIALIZED: bool = false;
pub static mut REJECTS: u64 = 0;
pub static mut ACCEPTS: u64 = 0;

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
    first_conventional_mem: u64,
    last_conventional_mem: u64,
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
            first_conventional_mem: 0,
            last_conventional_mem: 0,
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
        debug!("{}mb is the total memory size", mem_sz / 1024 / 1024);
        self.free = mem_sz as i64;
        // One for each page
        let bitmap_size = mem_sz / PAGE_SIZE / 8 + 1;
        debug!("{} is the size of the bitmap", bitmap_size);

        // Initialize Bitmap
        self.initialize_bitmap(bitmap_size as usize, current_largest_free_segment);
        self.lock_pages(self.bitmap.base, self.bitmap.size / PAGE_SIZE as usize + 1);

        let mut first_conventional_mem = 0;
        let mut last_conventional_mem = 0;
        for i in 0..self.entries {
            let ent = self.map[i];
            if ent.ty != MemoryType::ConventialMemory {
                if ent.phys_base < first_conventional_mem {
                    first_conventional_mem = ent.phys_base
                } else if ent.phys_base > last_conventional_mem {
                    last_conventional_mem = ent.phys_base
                };
                self.reserve_pages(ent.phys_base, ent.page_count as usize);
            } else if ent.phys_base <= _KERNEL_OFFSET {
                self.reserve_pages(ent.phys_base, ent.page_count as usize);
            }
        }
        self.last_conventional_mem = last_conventional_mem;
        self.first_conventional_mem = first_conventional_mem;
        debug!("{} is the amount of free space", self.free);
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

    pub fn lock_page(&mut self, addr: u64) -> bool {
        let idx = addr / PAGE_SIZE;

        // Already locked
        if self.bitmap[idx as usize] == true {
            return false;
        }

        if self.bitmap.set(idx as usize, true) {
            self.free -= PAGE_SIZE as i64;
            self.used += PAGE_SIZE as i64;
        } else {
            return false;
        }
        return true;
    }

    pub fn lock_pages(&mut self, addr: u64, count: usize) -> bool {
        for i in 0..count {
            if self.lock_page((addr + (i as u64 * PAGE_SIZE)) as u64) == false {
                return false;
            }
        }
        return true;
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
                self.lock_page(self.last_bmap_index * PAGE_SIZE);
                unsafe {
                    ACCEPTS += 1;
                }
                return self.last_bmap_index * PAGE_SIZE;
            }
            self.last_bmap_index += 1;
        }
        unsafe {
            REJECTS += 1;
        }
        panic!(
            "Out of Memory: {} out of {} pages were allocated.",
            unsafe { ACCEPTS },
            self.total_memory() * PAGE_SIZE
        );
    }

    pub fn alloc_aligned(&mut self, align: u64) -> u64 {
        let mut temp_index = self.last_bmap_index;
        while temp_index < (self.bitmap.size as u64 * 8 as u64) {
            if self.bitmap[temp_index as usize] == false {
                if is_aligned(temp_index * PAGE_SIZE, align) {
                    self.lock_page(temp_index * PAGE_SIZE);
                    unsafe {
                        ACCEPTS += 1;
                    }
                    return temp_index * PAGE_SIZE;
                }
            }
            temp_index += 1;
        }
        unsafe {
            REJECTS += 1;
        }
        panic!(
            "Out of Memory: {} out of {} pages were allocated.",
            unsafe { ACCEPTS },
            self.total_memory() * PAGE_SIZE
        );
    }

    pub fn allocate_from_addr_to_count_unchecked(&mut self, addr: u64, count: usize) -> bool {
        return if self.lock_pages(addr, count) == true {
            true
        } else {
            false
        };
    }

    pub fn total_memory(&self) -> u64 {
        unsafe {
            static mut MEM_SZ_BYTES: u64 = 0;
            // If calculated before, just return it
            if MEM_SZ_BYTES > 0 {
                return MEM_SZ_BYTES;
            }
            debug!("ENTries: {} : LENII {}", self.entries, self.map.len());
            for i in self.map.iter() {
                MEM_SZ_BYTES += i.page_count * PAGE_SIZE;
            }
            kprintln!(
                "MEM_SZ_BYTES: {} :: MEM_SZ_KILOBYTES: {} :: MEM_SZ_MEGABYTES: {}",
                MEM_SZ_BYTES,
                MEM_SZ_BYTES / 1024,
                MEM_SZ_BYTES / 1024 / 1024
            );
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
        let addr = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page();
        NonNull::new_unchecked(addr as *mut T).as_mut()
    }
}
