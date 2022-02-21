use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

use spin::Mutex;

use crate::{address_of, kprintln, memory::paging::page_frame_allocator::request_page};

pub static PAGE_TABLE_MANAGER: Mutex<MaybeUninit<PageTableManager>> =
    Mutex::new(MaybeUninit::uninit());

use super::super::memset;
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PageDescriptorEntry {
    entry: u64,
}

impl PageDescriptorEntry {
    #[inline]
    pub fn new() -> Self {
        Self { entry: 0 }
    }
    #[inline]
    pub const fn is_unused(&self) -> bool {
        self.entry == 0
    }

    pub fn set_unused(&mut self) {
        let _ = self.entry = 0;
    }

    #[inline]
    pub const fn flags(&self) -> PageTableFlag {
        PageTableFlag::from_bits_truncate(self.entry)
    }
    #[inline]
    pub fn addr(&self) -> u64 {
        (self.entry & 0x000f_ffff_ffff_f000) >> 12
    }
    #[inline]
    pub fn set_addr(&mut self, addr: u64) {
        let actual_addr = addr & 0x000000ffffffffff;
        self.entry &= 0xfff0000000000fff;
        self.entry |= actual_addr << 12;
    }
    #[inline]
    pub fn get_flag(&self, flag: PageTableFlag) -> bool {
        assert!((self.entry & flag.bits() > 0) == self.flags().contains(flag));
        return self.entry & flag.bits > 0;
    }

    #[inline]
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
        return f.finish();
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
#[repr(align(0x1000))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageTable {
    pub entries: [PageDescriptorEntry; ENTRIES],
}

impl Index<usize> for PageTable {
    type Output = PageDescriptorEntry;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl PageTable {
    pub fn get_non_empty<'arr>(&mut self) -> impl Iterator<Item = &PageDescriptorEntry> {
        let slice = self.entries.iter().filter(|f| !(*f).flags().is_empty());
        slice
    }

    pub fn print_non_empty(&mut self) {
        for item in self.get_non_empty() {
            crate::debug!("{:?}", item);
        }
    }
}

impl core::fmt::Debug for PageTable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.entries[..].fmt(f)
    }
}

pub struct PageMapIndexer {
    pub pdp_idx: usize,
    pub pd_idx: usize,
    pub pt_idx: usize,
    pub p_idx: usize,
}

impl PageMapIndexer {
    pub fn new(addr: u64) -> Self {
        let mut virtual_addr = (addr >> 12) as usize;
        let p_idx = virtual_addr & 0x1ff;
        virtual_addr >>= 9;
        let pt_idx = virtual_addr & 0x1ff;
        virtual_addr >>= 9;
        let pd_idx = virtual_addr & 0x1ff;
        virtual_addr >>= 9;
        let pdp_idx = virtual_addr & 0x1ff;
        assert!(p_idx <= 512);
        assert!(pt_idx <= 512);
        assert!(pd_idx <= 512);
        assert!(pdp_idx <= 512);
        Self {
            p_idx,
            pt_idx,
            pd_idx,
            pdp_idx,
        }
    }
}

#[repr(C)]
pub struct PageTableManager<'table> {
    pml4: &'table mut PageTable,
}

fn addr_to_page_table<'retval>(addr: u64) -> &'retval mut PageTable {
    unsafe { &mut *(addr as *mut u64 as *mut PageTable) }
}

impl<'table> PageTableManager<'table> {
    pub fn new(pml4: &'table mut PageTable) -> Self {
        Self { pml4: pml4 }
    }

    pub fn map_memory(&mut self, virtual_mem: u64, physical_mem: u64) {
        let indexer = PageMapIndexer::new(virtual_mem);
        // First Page
        const SET_PT_TO_NULL: fn(&mut PageTable) = |pt| unsafe {
            memset(address_of!(pt), 0, 0x1000);
        };

        // Map PDP
        let mut pdp_pde = self.pml4[indexer.pdp_idx];
        let pdp = if !pdp_pde.get_flag(PageTableFlag::PRESENT) {
            let tmp = request_page::<PageTable>();
            SET_PT_TO_NULL(tmp);
            pdp_pde.set_addr(address_of!(tmp) >> 12);
            pdp_pde.set_flag(PageTableFlag::PRESENT, true);
            pdp_pde.set_flag(PageTableFlag::READ_WRITE, true);
            self.pml4[indexer.pdp_idx] = pdp_pde;
            tmp
        } else {
            addr_to_page_table(pdp_pde.addr() << 12)
        };

        // Map PageDirectoryEntry
        let mut pd_pde = pdp.entries[indexer.pd_idx];
        let pd = if !pd_pde.get_flag(PageTableFlag::PRESENT) {
            let tmp = request_page();
            SET_PT_TO_NULL(tmp);
            pd_pde.set_addr(address_of!(tmp) >> 12);
            pd_pde.set_flag(PageTableFlag::PRESENT, true);
            pd_pde.set_flag(PageTableFlag::READ_WRITE, true);
            pdp.entries[indexer.pd_idx] = pd_pde;
            tmp
        } else {
            addr_to_page_table(pd_pde.addr() << 12)
        };

        // Map PageTable
        let mut pt_pde = pd.entries[indexer.pt_idx];
        let pt = if !pt_pde.get_flag(PageTableFlag::PRESENT) {
            let tmp = request_page();
            SET_PT_TO_NULL(tmp);
            pt_pde.set_addr(address_of!(tmp) >> 12);
            pt_pde.set_flag(PageTableFlag::PRESENT, true);
            pt_pde.set_flag(PageTableFlag::READ_WRITE, true);
            pd.entries[indexer.pt_idx] = pt_pde;
            tmp
        } else {
            addr_to_page_table(pt_pde.addr() << 12)
        };

        let mut page_pde = pt.entries[indexer.p_idx];
        page_pde.set_addr(physical_mem >> 12);
        page_pde.set_flag(PageTableFlag::PRESENT, true);
        page_pde.set_flag(PageTableFlag::READ_WRITE, true);

        //pt.entries[indexer.p_idx] = page_pde; // FIXME: This causes a Page Fault
    }
}

core::arch::global_asm!(include_str!("paging.s"));

extern "C" {
    pub fn upload_pml4(addr: u64);
}
