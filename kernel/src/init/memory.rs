use bks::{Handover, PAGE_SIZE};

use crate::heap::{free, malloc_ptr, Heap, GLOBAL_HEAP};
use crate::memory::memset;
use crate::memory::paging::page_frame_allocator::{request_page, ACCEPTS, REJECTS};
use crate::memory::paging::page_table_manager::{upload_pml4, PageTable, PageTableManager};
use crate::{
    kprintln,
    memory::paging::page_frame_allocator::{PageFrameAllocator, PAGE_FRAME_ALLOCATOR},
};
use crate::{HEAP_ADDRESS, HEAP_LENGTH};
use core::arch::asm;

// Defined in Linker Script
#[no_mangle]
static _KERNEL_START: u64 = 0;

// Defined in Linker Script
#[no_mangle]
static _KERNEL_END: u64 = 0;

// Defined in Linker Script
#[no_mangle]
pub static _KERNEL_OFFSET: u64 = 0;

/// Initializes the memory (Paging, Heap, etc)
pub fn init_paging(handover: &mut Handover) {
    kprintln!("Preparing Memory");
    unsafe {
        kprintln!(
            "ENT: {}, SZ: {}, ENTSZ: {}",
            handover.mmap_entries,
            handover.mmap_size,
            handover.mmap_entry_size
        );
        // Set the Global PageFrameAllocator
        PAGE_FRAME_ALLOCATOR.lock().write(PageFrameAllocator::new(
            handover.raw_memory_map() as *mut u8,
            handover.mmap_entries,
            handover.mmap_size,
            handover.mmap_entry_size,
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

            // Step through the memory mapping phys x -> virt x
            let total_mem = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().total_memory();
            kprintln!(
                "Mapping Memory ({} bytes, {}kb, {}mb)...",
                total_mem,
                total_mem / 1024,
                total_mem / 1024 / 1024,
            );
            //for i in (_KERNEL_OFFSET..(total_mem)).step_by(0x1000) {
            //    page_table_manager.map_memory(i as u64, i as u64);
            //} FIXME: Very Slow

            kprintln!("Mapped Memory");
            kprintln!("{}", REJECTS);
            kprintln!("Mapping Framebuffer...");
            kprintln!("ACC: {}, FULL: {}", ACCEPTS, total_mem / 0x1000);
            let fb_base = handover.framebuffer().base;
            let fb_size = handover.framebuffer().size + PAGE_SIZE as usize;
            PAGE_FRAME_ALLOCATOR
                .lock()
                .assume_init_mut()
                .lock_pages(fb_base, fb_size / PAGE_SIZE as usize + 1);
            let fb_end = fb_base + fb_size as u64;
            //for i in (fb_base..fb_end).step_by(PAGE_SIZE as usize) {
            //    page_table_manager.map_memory(i, i);
            //    if REJECTS > 0 {
            //        kprintln!("{}", REJECTS);
            //    }
            //} FIXME: Mapping the FrameBuffer causes a general protection fault
            kprintln!("Setting default PML4");
            let addr = pml4 as *mut PageTable as *mut u64 as u64;
            // FIXME: upload_pml4(addr);
            //kprintln!("Finished preparing memory!");
        }
    }
}

pub fn init_heap(handover: &mut Handover) {
    kprintln!("Initializing Heap!");
    unsafe {
        crate::heap::GLOBAL_HEAP
            .lock()
            .write(Heap::new(HEAP_ADDRESS, HEAP_LENGTH));
        let alloc = malloc_ptr::<u64>(0x8000);
        *alloc = 26;
        kprintln!("{}", *alloc);

        let alloc_b = malloc_ptr::<u64>(0x8000);
        *alloc_b = 42;
        kprintln!("{}", *alloc_b);

        let alloc_c = malloc_ptr::<u64>(0x8000);
        *alloc_c = 28;
        //kprintln!("{}", *alloc_c);

        {
            let ptr = malloc_ptr::<u64>(0x1000);
            kprintln!("{:p}", ptr);
            free(ptr as u64);
        }

        {
            let ptr = malloc_ptr::<u64>(0x1000);
            kprintln!("{:p}", ptr);
            free(ptr as u64);
        }

        {
            let ptr = malloc_ptr::<u64>(0x1000);
            kprintln!("{:p}", ptr);
            free(ptr as u64);
        }

        kprintln!("{}", *alloc);
        kprintln!("L");
    }
}
