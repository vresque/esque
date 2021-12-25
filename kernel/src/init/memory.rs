use bks::{Handover, PAGE_SIZE};

use crate::memory::memset;
use crate::memory::paging::page_frame_allocator::REJECTS;
use crate::memory::paging::page_table_manager::{PageTable, PageTableManager};
use crate::{
    kprintln,
    memory::paging::page_frame_allocator::{PageFrameAllocator, PAGE_FRAME_ALLOCATOR},
};
use core::arch::asm;

// Defined in Linker Script
#[no_mangle]
static _KERNEL_START: u64 = 0;

// Defined in Linker Script
#[no_mangle]
static _KERNEL_END: u64 = 0;

/// Initializes the memory (Paging, Heap, etc)
pub fn init_paging(handover: &mut Handover) {
    kprintln!("Preparing Memory");
    unsafe {
        // Set the Global PageFrameAllocator
        PAGE_FRAME_ALLOCATOR.lock().write(PageFrameAllocator::new(
            handover.raw_memory_map() as *mut u8,
            handover.mmap_entries,
            handover.mmap_size,
        ));
        // "Initialize" the PageFrameAllocator
        PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .read_memory_map();
        return;
    }
}

pub fn map_memory(handover: &mut Handover) {
    unsafe {
        {
            let kernel_size = _KERNEL_END - _KERNEL_START;
            let kernel_pages = kernel_size / PAGE_SIZE + 1;
            // Lock the memory at which the kernel has been loaded
            PAGE_FRAME_ALLOCATOR
                .lock()
                .assume_init_mut()
                .lock_pages(_KERNEL_START, kernel_pages as usize);
        }

        {
            // The PageMapLevel4
            let pml4 = &mut *(PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page()
                as *mut u64 as *mut PageTable);
            memset(
                pml4 as *mut PageTable as *mut u64 as u64,
                0,
                PAGE_SIZE as usize,
            );

            let page_table_manager = &mut PageTableManager::new(*pml4);

            kprintln!("Mapping Memory...");
            // Step through the memory mapping phys x -> virt x
            let total_mem = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().total_memory();
            for i in (0..(total_mem)).step_by(0x1000) {
                page_table_manager.map_memory(i as u64, i as u64 + _KERNEL_START);
            }
            kprintln!("Mapped Memory");
            kprintln!("{}", REJECTS);
            kprintln!("Mapping Framebuffer...");
            let fb_base = handover.framebuffer().base;
            let fb_size = handover.framebuffer().size + PAGE_SIZE as usize;
            PAGE_FRAME_ALLOCATOR
                .lock()
                .assume_init_mut()
                .lock_pages(fb_base, fb_size / PAGE_SIZE as usize + 1);

            let fb_end = fb_base + fb_size as u64;
            for i in (fb_base..fb_end).step_by(PAGE_SIZE as usize) {
                page_table_manager.map_memory(i, i);
                if REJECTS > 0 {
                    kprintln!("{}", REJECTS);
                }
            }
            kprintln!("Setting default PML4");
            asm!("mov cr3, {}", in(reg) (pml4 as *mut PageTable));
            kprintln!("Finished preparing memory!");
        }
    }
}

pub fn init_heap(handover: &mut Handover) {
    kprintln!("Initializing Heap!");
}