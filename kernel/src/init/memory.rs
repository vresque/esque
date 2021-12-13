use bks::{Handover, PAGE_SIZE};

use crate::framebuffer::FRAMEBUFFER_GUARD;
use crate::kinfo;
use crate::memory::bitmap::Bitmap;
use crate::memory::paging::page_table_manager::{PageMapIndexer, PageTable, PageTableManager};
use crate::{
    kprintln,
    memory::paging::page_frame_allocator::{PageFrameAllocator, PAGE_FRAME_ALLOCATOR},
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

        kprintln!("Setting up Page Table Manager");

        let mut page_table_lvl_4 = *(PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page()
            as *mut u64 as *mut PageTable);
        rlibc::memset(
            &mut page_table_lvl_4 as *mut PageTable as *mut u8,
            0,
            0x1000,
        );
        let page_table_manager = &mut PageTableManager::new(page_table_lvl_4);

        kprintln!("Mapping Memory...");
        let mut i = 0;
        let total = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().total_memory();
        for _ in 0..(total) {
            // Virtual = Physical memory
            page_table_manager.map_memory(i, i);
            // Each Page
            i += 0x1000;
        }
        kprintln!("Remapping Framebuffer");
        // Re-map framebuffer
        let base = FRAMEBUFFER_GUARD
            .lock()
            .assume_init_mut()
            .framebuffer()
            .base;
        let size = FRAMEBUFFER_GUARD
            .lock()
            .assume_init_mut()
            .framebuffer()
            .size
            + 0x1000; // Plus One Page
        let mut t = 0;
        for _ in base..(base + size as u64) {
            page_table_manager.map_memory(t, t);
            t += 0x1000;
        }

        kprintln!("If i die - it shall be during the page initialization");
        // Load Page Table
        let points = &mut page_table_lvl_4 as *mut PageTable;
        asm!("mov {}, cr3", in(reg) points);
        kprintln!("Still alive!");
    };
}
