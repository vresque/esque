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

    pub fn set(&mut self, idx: usize, value: bool) -> bool {
        if idx > self.size * 8 {
            return false;
        };
        let byte_index = (idx / 8) as usize;
        let bit_index = (idx % 8) as u8;
        let bit_indexer = (0b10000000 >> bit_index) as u8;
        unsafe {
            let ptr = self.base as *mut u8;
            *ptr.add(byte_index) &= !bit_indexer;
            if value == true {
                *ptr.add(byte_index) |= bit_indexer;
            }
        }
        true
    }
}

impl core::fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Bitmap ")?;
        writeln!(f, "  size: {:#x?}", self.size)?;
        writeln!(f, "  values: [")?;
        for i in 0..(self.size * 8) {
            write!(f, "{}, ", self[i])?;
        }

        Ok(())
    }
}

impl core::ops::Index<usize> for Bitmap {
    type Output = bool;
    fn index(&self, idx: usize) -> &Self::Output {
        if idx > self.size * 8 {
            return &false;
        }
        let byte_index = (idx / 8) as usize;
        let bit_index = (idx % 8) as u8;
        let bit_indexer = 0b10000000 >> bit_index;
        unsafe {
            let ptr = self.base as *mut u8;
            let val = *ptr.add(byte_index) & bit_indexer;
            if val > 0 {
                return &true;
            } else {
                return &false;
            }
        }
    }
}
