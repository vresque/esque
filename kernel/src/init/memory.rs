use bks::{Handover, PAGE_SIZE};
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::structures::paging::PhysFrame;
use x86_64::PhysAddr;

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
        kprintln!("{:#x?}", _KERNEL_START);
        let kernel_size = _KERNEL_END - _KERNEL_START;
        let kernel_pages = kernel_size / PAGE_SIZE + 1;
        PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .lock_pages(_KERNEL_START, kernel_pages as usize);

        let mut pml4 = &mut *(PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page()
            as *mut u64 as *mut PageTable);
        memset(
            pml4 as *mut PageTable as *mut u64 as u64,
            0,
            PAGE_SIZE as usize,
        );

        let page_table_manager = &mut PageTableManager::new(*pml4);

        kprintln!("Mapping Memory...");
        let total_mem = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().total_memory();
        for i in (0..(total_mem)).step_by(0x1000) {
            page_table_manager.map_memory(i as u64, i as u64 + _KERNEL_START);
            if rejects > 0 {
                kprint!("-> {} -> ", rejects);
            }
        }
        kprintln!("Mapped Memory");

        kprintln!("Mapping Framebuffer...");
        let fb_base = handover.framebuffer().base;
        let fb_size = handover.framebuffer().size + PAGE_SIZE as usize;
        PAGE_FRAME_ALLOCATOR
            .lock()
            .assume_init_mut()
            .lock_pages(fb_base, fb_size / PAGE_SIZE as usize + 1);

        for i in (fb_base..(fb_base + fb_size as u64)).step_by(PAGE_SIZE as usize) {
            page_table_manager.map_memory(i, i);
        }
        kprintln!("Setting default PML4");
        let mut old_map = pml4.clone();
        let mut page_map_lvl_4_ptr = pml4 as *mut PageTable;
        Cr3::write(
            PhysFrame::containing_address(PhysAddr::new(page_map_lvl_4_ptr as *mut u64 as u64)),
            Cr3Flags::PAGE_LEVEL_WRITETHROUGH,
        );
        assert_eq!(page_map_lvl_4_ptr, &mut old_map as *mut PageTable);
        //page_table_manager.map_memory(0x600000000, 0x80000);
        //kprintln!("Finished preparing memory!");
    } //
} //
