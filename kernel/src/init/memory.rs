use bks::{Handover, PAGE_SIZE};

use crate::framebuffer::{Color, FRAMEBUFFER_GUARD};
use crate::memory::bitmap::Bitmap;
use crate::memory::memset;
use crate::memory::paging::page_frame_allocator::rejects;
use crate::memory::paging::page_table_manager::{PageMapIndexer, PageTable, PageTableManager};
use crate::{kinfo, kprint};
use crate::{
    kprintln,
    memory::paging::page_frame_allocator::{PageFrameAllocator, PAGE_FRAME_ALLOCATOR},
};

#[no_mangle]
static _KERNEL_START: u64 = 0;

#[no_mangle]
static _KERNEL_END: u64 = 0;

pub fn init_memory(handover: &mut Handover) {
    kprintln!("Preparing Memory");
    unsafe {
        PAGE_FRAME_ALLOCATOR.lock().write(PageFrameAllocator::new(
            handover.raw_memory_map() as *mut u8,
            handover.mmap_entries,
            handover.mmap_size,
        ));
        PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .read_memory_map();

        let kernel_size = _KERNEL_END - _KERNEL_START;
        let kernel_pages = kernel_size / PAGE_SIZE + 1;
        PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .lock_pages(_KERNEL_START, kernel_pages as usize);

        let pml4 = &mut *(PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page() as *mut u64
            as *mut PageTable);
        memset(
            pml4 as *mut PageTable as *mut u64 as u64,
            0,
            PAGE_SIZE as usize,
        );

        let page_table_manager = &mut PageTableManager::new(*pml4);
        kprintln!("Mapping Memory...");
        let total_mem = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().total_memory();
        for i in (0..(total_mem)).step_by(0x1000) {
            assert_eq!(i % 0x1000, 0);
            kprint!("{:?} ", rejects);
            page_table_manager.map_memory(i as u64, i as u64);
        }
        kprintln!("Mapped Memory");
    }
}
