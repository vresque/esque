use bks::{Handover, PAGE_SIZE};
use spin::Mutex;

use crate::heap::{free, malloc_ptr, Heap};
use crate::memory::paging::page_frame_allocator::{request_page, ACCEPTS, REJECTS};
use crate::memory::paging::page_table_manager::{PageTable, PageTableManager, PAGE_TABLE_MANAGER};
use crate::{address_of, debug, info, kprint, success, HEAP_ADDRESS, HEAP_LENGTH};
use crate::{
    kprintln,
    memory::paging::page_frame_allocator::{PageFrameAllocator, PAGE_FRAME_ALLOCATOR},
};
use core::arch::asm;
use core::mem::MaybeUninit;

// Defined in Linker Script
#[no_mangle]
pub static _KERNEL_START: u64 = 0;

// Defined in Linker Script
#[no_mangle]
pub static _KERNEL_END: u64 = 0;

// Defined in Linker Script
#[no_mangle]
pub static _KERNEL_OFFSET: u64 = 0;

/// Initializes the memory (Paging, Heap, etc)
pub fn init_paging(handover: &mut Handover) {
    info!("Preparing Memory");
    unsafe {
        debug!(
            "ENT: {}, SZ: {}, ENTSZ: {}",
            handover.mmap_entries, handover.mmap_size, handover.mmap_entry_size
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

        PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .lock_pages(handover.initramfs_base, handover.initramfs_size);
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
            let mut pml4: &mut PageTable;
            {
                let value: u64;
                asm!("mov {}, cr3", out(reg) value, options(nomem, nostack, preserves_flags));
                let addr = value & 0x_000f_ffff_ffff_f000;
                pml4 = &mut *(addr as *mut u64 as *mut PageTable);
            }

            let pml4_addr = pml4 as *const PageTable as u64;
            let mut page_table_manager = PageTableManager::new(pml4);

            // Step through the memory mapping phys x -> virt x
            let total_mem = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().total_memory();
            debug!(
                "Mapping Memory ({} bytes, {}kb, {}mb)...",
                total_mem,
                total_mem / 1024,
                total_mem / 1024 / 1024,
            );
            for i in (_KERNEL_OFFSET..(total_mem)).step_by(0x1000) {
                &mut page_table_manager.map_memory(i as u64, i as u64);
            }

            debug!("Mapped Memory");
            debug!("{}", REJECTS);
            info!("Mapping Framebuffer...");
            debug!("ACC: {}, FULL: {}", ACCEPTS, total_mem / 0x1000);
            let fb_base = handover.framebuffer().base;
            let fb_size = handover.framebuffer().size + PAGE_SIZE as usize;
            PAGE_FRAME_ALLOCATOR
                .lock()
                .assume_init_mut()
                .lock_pages(fb_base, fb_size / PAGE_SIZE as usize + 1);
            let fb_end = fb_base + fb_size as u64;
            for i in (fb_base..fb_end).step_by(PAGE_SIZE as usize) {
                &mut page_table_manager.map_memory(i, i);
                if REJECTS > 0 {
                    kprintln!("{}", REJECTS);
                }
            }
            info!("Setting default PML4");

            //debug!("{:x?}", &*(pml4_addr as *mut PageTable));
            page_table_manager.map_memory(0xfffffffffff, 0x9000);

            let value = pml4_addr | 0; //(1 << 3) as u64;
                                       //asm!("mov cr3, {}", in(reg) value, options(nostack, preserves_flags));
            let xyy = 0xfffffffffff as *mut u64;
            *xyy = 24;
            debug!("{}", *xyy);

            PAGE_TABLE_MANAGER.lock().write(page_table_manager);
            success!("Finished preparing memory!");
        }
    }
}

pub fn init_heap(_handover: &mut Handover) {
    info!("Initializing Heap!");
    unsafe {
        crate::heap::GLOBAL_HEAP
            .lock()
            .write(Heap::new(HEAP_ADDRESS, HEAP_LENGTH));
        let alloc = malloc_ptr::<u64>(0x8000);
        *alloc = 26;
        debug!("{}", *alloc);

        let alloc_b = malloc_ptr::<u64>(0x8000);
        *alloc_b = 42;
        debug!("{}", *alloc_b);

        let alloc_c = malloc_ptr::<u64>(0x8000);
        *alloc_c = 28;
        //kprintln!("{}", *alloc_c);

        {
            let ptr = malloc_ptr::<u64>(0x1000);
            debug!("{:p}", ptr);
            free(ptr as u64);
        }

        {
            let ptr = malloc_ptr::<u64>(0x1000);
            debug!("{:p}", ptr);
            free(ptr as u64);
        }

        {
            let ptr = malloc_ptr::<u64>(0x1000);
            debug!("{:p}", ptr);
            free(ptr as u64);
        }

        debug!("{}", *alloc);
        debug!("L");
    }
}
