use core::mem::MaybeUninit;

use bks::EfiMemoryDescriptor;
use spin::Mutex;

use crate::kprintln;

pub static BITMAP: Mutex<MaybeUninit<Bitmap<EfiMemoryDescriptor>>> =
    Mutex::new(MaybeUninit::uninit());

#[derive(Debug)]
pub struct Bitmap<T>
where
    T: 'static,
{
    base: u64,
    map: &'static mut [T],
    entries: usize,
    size: usize,
}

impl<T> Bitmap<T> {
    pub fn new(base: *mut u8, entries: usize, size: usize) -> Self {
        let array = unsafe { core::slice::from_raw_parts_mut(base as *mut u8 as *mut T, entries) };
        Self {
            base: base as u64,
            map: array,
            entries: entries,
            size,
        }
    }

    pub fn set(&mut self, idx: usize, value: bool) {
        let byte_index = idx / 8;
        let bit_index = idx % 8;
        let indexer = 0b10000000 >> bit_index;
        unsafe {
            (*(self.base as *mut u8).add(byte_index)) &= !indexer;
            if value {
                (*(self.base as *mut u8).add(byte_index)) |= indexer
            }
        };
    }
}

impl<T> core::ops::Index<usize> for Bitmap<T> {
    type Output = bool;
    fn index(&self, idx: usize) -> &Self::Output {
        let byte_index = idx / 8;
        let bit_index = idx % 8;
        let indexer = 0b10000000 >> bit_index;
        unsafe {
            if ((*(self.base as *mut u8).add(byte_index)) & indexer) > 0 {
                return &true;
            } else {
                return &false;
            }
        }
    }
}

impl Bitmap<EfiMemoryDescriptor> {
    pub fn total_memory(&mut self) -> u64 {
        static mut mem_sz_bytes: u64 = 0;
        // If calculated before, just return it
        if unsafe { mem_sz_bytes } > 0 {
            unsafe {
                return mem_sz_bytes;
            }
        }
        let mut sum_of_mem_sizes = 0;
        for i in self.map.iter() {
            sum_of_mem_sizes += i.page_count;
        }

        unsafe {
            mem_sz_bytes = sum_of_mem_sizes;
            mem_sz_bytes
        }
    }
}
