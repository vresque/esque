use bks::{Handover, PAGE_SIZE};

use crate::heap::Heap;
use crate::memory::paging::page_table_manager::{PageTable, PageTableManager, PAGE_TABLE_MANAGER};
use crate::{arch::HEAP_ADDRESS, arch::HEAP_LENGTH, debug, info, kprint, success};
use crate::{
    kprintln,
    memory::paging::page_frame_allocator::{PageFrameAllocator, PAGE_FRAME_ALLOCATOR},
};
use core::arch::asm;

// Defined in Linker Script
#[no_mangle]
pub static _KERNEL_END: u64 = 0;

// Defined in Linker Script
#[no_mangle]
pub static _KERNEL_START: u64 = 0;

// Defined in Linker Script
#[no_mangle]
pub static _KERNEL_OFFSET: u64 = 0;

/// Initializes the memory (Paging, Heap, etc)
pub fn init_initial_paging(handover: &mut Handover) {
    info!("Preparing Memory");
    unsafe {
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
        debug!("{_KERNEL_END}, {_KERNEL_START}, {_KERNEL_OFFSET}");
        return;
        {
            let kernel_size = _KERNEL_END - _KERNEL_START;
            let kernel_pages = kernel_size / PAGE_SIZE + 1;
            // Lock the memory at which the kernel has been loaded
            PAGE_FRAME_ALLOCATOR
                .lock()
                .assume_init_mut()
                .lock_pages(_KERNEL_START, kernel_pages as usize);
        }
        return;
        {
            // The PageMapLevel4
            let pml4: &mut PageTable;
            {
                let value: u64;
                asm!("mov {}, cr3", out(reg) value, options(nomem, nostack, preserves_flags));
                let addr = &mut *(value as *mut u64 as *mut PageTable);
                pml4 = addr;
            }

            //memset(address_of!(pml4), 0, bks::PAGE_SIZE as usize);
            let pml4_addr = pml4 as *const PageTable as u64;
            let mut page_table_manager = PageTableManager::new(pml4);
            // Mapping (and locking) the framebuffer
            info!("Mapping Framebuffer...");
            let fb_base = handover.framebuffer().base;
            let fb_size = handover.framebuffer().size + PAGE_SIZE as usize;
            PAGE_FRAME_ALLOCATOR
                .lock()
                .assume_init_mut()
                .lock_pages(fb_base, fb_size / PAGE_SIZE as usize + 1);
            let fb_end = fb_base + fb_size as u64;
            for i in (fb_base..fb_end).step_by(PAGE_SIZE as usize) {
                let _ = &mut page_table_manager.map_memory(i, i);
            }

            // Step through the memory mapping phys x -> virt x
            let total_mem = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().total_memory();
            debug!(
                "Mapping Memory ({} bytes, {}kb, {}mb)...",
                total_mem,
                total_mem / 1024,
                total_mem / 1024 / 1024,
            );

            for i in (_KERNEL_START..(total_mem)).step_by(0x1000) {
                if (fb_base..fb_end).contains(&i) {
                    continue;
                } // No double mapping of the framebuffer
                let _ = &mut page_table_manager.map_memory(i as u64, i as u64);
            }
            debug!("Mapped Memory");

            info!("Setting default PML4");

            //debug!("{:x?}", &*(pml4_addr as *mut PageTable));

            let value = pml4_addr | 0; //(1 << 3) as u64;
            asm!("mov cr3, {}", in(reg) value, options(nostack, preserves_flags));

            PAGE_TABLE_MANAGER.lock().write(page_table_manager);
            success!("Finished preparing memory!");

            PAGE_TABLE_MANAGER
                .lock()
                .assume_init_mut()
                .map_memory(0x6_0000_0000, 0x800_000);
            //let xyy = 0x6_0000_0000 as *mut u64;
            //*xyy = 24;
            //debug!("{}", *xyy);
        }
    }
}
