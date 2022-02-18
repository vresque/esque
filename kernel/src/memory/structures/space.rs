use crate::memory::paging::{
    page_frame_allocator::PAGE_FRAME_ALLOCATOR, page_table_manager::PageTable,
};

use super::{Frame, PhysicalAddress, VirtualAddress};

/// # Memory Space
/// This structure represents a *virtual* address space.
/// This space contains a reference to the page table
pub struct Space {
    cr3: Frame,
}

impl Space {
    pub fn new() -> Option<Self> {
        unsafe {
            let frame = Frame::which_contains(PhysicalAddress::new(
                PAGE_FRAME_ALLOCATOR
                    .lock()
                    .assume_init_mut()
                    .alloc_aligned(bks::PAGE_SIZE),
            ));
            let addr = frame.start();
            let virt: VirtualAddress = VirtualAddress::new(addr.as_u64());
            let table = &mut *virt.as_mut_ptr::<PageTable>();
        }

        None
    }
}
