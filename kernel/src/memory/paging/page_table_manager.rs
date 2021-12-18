use core::ops::{Index, IndexMut};

use crate::kprintln;

use super::super::memset;
use super::page_frame_allocator::PAGE_FRAME_ALLOCATOR;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PageDescriptorEntry {
    entry: u64,
}

impl PageDescriptorEntry {
    pub fn new() -> Self {
        Self { entry: 0 }
    }
    #[inline]
    pub const fn is_unused(&self) -> bool {
        self.entry == 0
    }
    pub const fn flags(&self) -> PageTableFlag {
        PageTableFlag::from_bits_truncate(self.entry)
    }
    pub fn addr(&self) -> u64 {
        self.entry & 0x000f_ffff_ffff_f000
    }

    pub fn set_addr(&mut self, addr: u64) {
        let actual_addr = addr & 0x000000ffffffffff;
        self.entry &= 0xfff0000000000fff;
        self.entry |= actual_addr << 12;
    }

    pub fn get_flag(&mut self, flag: PageTableFlag) -> bool {
        return if self.entry & flag.bits() > 0 {
            true
        } else {
            false
        };
    }

    pub fn set_flag(&mut self, flag: PageTableFlag, enabled: bool) {
        self.entry &= !flag.bits();
        if enabled {
            self.entry |= flag.bits()
        }
    }
}

impl core::fmt::Debug for PageDescriptorEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut f = f.debug_struct("PageTableEntry");
        f.field("addr", &self.addr());
        f.field("flags", &self.flags());
        f.finish()
    }
}

bitflags::bitflags! {
    pub struct PageTableFlag: u64 {
        const PRESENT = 1;
        const READ_WRITE = 1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH = 1 << 3;
        const NO_CACHE = 1 << 4;
        const ACCESSED = 1 << 5;
        const DIRTY = 1 << 6;
        const LARGE_PAGE = 1 << 7;
        const GLOBAL = 1 << 8;
        const BIT_9 = 1 << 9;
        const BIT_10 = 1 << 10;
        const BIT_11 = 1 << 11;
        const BIT_52 = 1 << 52;
        const BIT_53 = 1 << 53;
        const BIT_54 = 1 << 54;
        const BIT_55 = 1 << 55;
        const BIT_56 = 1 << 56;
        const BIT_57 = 1 << 57;
        const BIT_58 = 1 << 58;
        const BIT_59 = 1 << 59;
        const BIT_60 = 1 << 60;
        const BIT_61 = 1 << 61;
        const BIT_62 = 1 << 62;
        const NO_EXECUTE = 1 << 63;
    }
}

const ENTRIES: usize = 512;
#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct PageTable {
    entries: [PageDescriptorEntry; ENTRIES],
}

impl Index<usize> for PageTable {
    type Output = PageDescriptorEntry;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl core::fmt::Debug for PageTable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.entries[..].fmt(f)
    }
}

pub struct PageMapIndexer {
    pub pdp_idx: u64,
    pub pd_idx: u64,
    pub pt_idx: u64,
    pub p_idx: u64,
}

impl PageMapIndexer {
    pub fn new(addr: u64) -> Self {
        let mut virtual_addr = addr >> 12;
        let p_idx = virtual_addr & 0x1ff;
        virtual_addr >>= 9;
        let pt_idx = virtual_addr & 0x1ff;
        virtual_addr >>= 9;
        let pd_idx = virtual_addr & 0x1ff;
        virtual_addr >>= 9;
        let pdp_idx = virtual_addr & 0x1ff;
        Self {
            p_idx,
            pt_idx,
            pd_idx,
            pdp_idx,
        }
    }
}

#[repr(C)]
pub struct PageTableManager {
    pml4: PageTable,
}

impl PageTableManager {
    pub fn new(pml4: PageTable) -> Self {
        Self { pml4: pml4 }
    }

    pub fn map_memory(&mut self, virtual_mem: u64, physical_mem: u64) {
        let indexer = PageMapIndexer::new(virtual_mem);
        // First Page
        unsafe {
            // First Page
            let mut pde = self.pml4[indexer.pdp_idx as usize];
            let page = PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page();

            // ------ 1 ------------------
            let pdp: &mut PageTable = if !pde.get_flag(PageTableFlag::PRESENT) {
                let pdp = &mut *(page as *mut u64 as *mut PageTable);
                memset(pdp as *mut PageTable as *mut u64 as u64, 0, 0x1000);
                pde.set_addr((pdp as *mut PageTable as *mut u64 as u64) >> 12);
                pde.set_flag(PageTableFlag::PRESENT, true);
                pde.set_flag(PageTableFlag::READ_WRITE, true);
                self.pml4[indexer.pdp_idx as usize] = pde;
                pdp
            } else {
                let pdp = &mut *((pde.addr() << 12) as *mut PageTable);
                pdp
            };

            // ------------- PAGE DESCRIPTOR ---------
            pde = pdp[indexer.pd_idx as usize];
            let pd: &mut PageTable = if !pde.get_flag(PageTableFlag::PRESENT) {
                let pd = &mut *(PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page()
                    as *mut u64 as *mut PageTable);
                memset(pd as *mut PageTable as *mut u64 as u64, 0, 0x1000);
                pde.set_addr((pd as *mut PageTable as *mut u64 as u64) >> 12);
                pde.set_flag(PageTableFlag::PRESENT, true);
                pde.set_flag(PageTableFlag::READ_WRITE, true);
                pdp[indexer.pd_idx as usize] = pde;
                pd
            } else {
                let pd = &mut *((pde.addr() << 12) as *mut PageTable);
                pd
            };

            // -------------- Page Table (PT)
            pde = pd[indexer.pt_idx as usize];
            let pt: &mut PageTable = if !pde.get_flag(PageTableFlag::PRESENT) {
                let pt = &mut *(PAGE_FRAME_ALLOCATOR.lock().assume_init_mut().request_page()
                    as *mut u64 as *mut PageTable);
                memset(pt as *mut PageTable as *mut u64 as u64, 0, 0x1000);
                pde.set_addr((pt as *mut PageTable as *mut u64 as u64) >> 12);
                pde.set_flag(PageTableFlag::PRESENT, false);
                pde.set_flag(PageTableFlag::READ_WRITE, true);
                pd[indexer.pt_idx as usize] = pde;
                pt
            } else {
                let pt = &mut *((pde.addr() << 12) as *mut PageTable);
                pt
            };
            // Page at the end
            pde = pt[indexer.p_idx as usize];
            pde.set_addr(physical_mem >> 12);
            pde.set_flag(PageTableFlag::PRESENT, true);
            pde.set_flag(PageTableFlag::READ_WRITE, true);
            pt[indexer.pt_idx as usize] = pde;
        }; //
    }
}
