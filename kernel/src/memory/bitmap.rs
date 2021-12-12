use core::mem::MaybeUninit;

use bks::EfiMemoryDescriptor;
use spin::Mutex;

use crate::kprintln;

#[derive(Debug)]
pub struct Bitmap {
    pub base: u64,
    pub size: usize,
}

impl Bitmap {
    pub fn new(base: *mut u8, size: usize) -> Self {
        Self {
            base: base as u64,
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

impl core::ops::Index<usize> for Bitmap {
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
