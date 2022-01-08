use arrayvec::ArrayString;

pub const BLOCK_SIZE: usize = 512;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct PosixHeader {
    pub name: [u8; 100],
    pub mode: [u8; 8],
    pub uid: [u8; 8],
    pub gid: [u8; 8],
    pub size: [u8; 12],
    pub mtime: [u8; 12],
    pub chksum: [u8; 8],
    pub typeflag: u8,
    pub linkname: [u8; 100],
    pub magic: [u8; 6],
    pub version: [u8; 2],
    pub uname: [u8; 32],
    pub gname: [u8; 32],
    pub devmajor: [u8; 8],
    pub devminor: [u8; 8],
    pub prefix: [u8; 155],
}

impl PosixHeader {
    pub fn block_count(&self) -> usize {
        let size = octal_ascii_size_as_usize(self.size);
        let modulo = size % BLOCK_SIZE as usize;
        if modulo > 0 {
            (size / BLOCK_SIZE as usize) + 1
        } else {
            size / BLOCK_SIZE
        }
    }

    pub fn is_null(&self) -> bool {
        unsafe {
            core::slice::from_raw_parts(self as *const Self as *const u8, BLOCK_SIZE)
                .iter()
                .filter(|x| **x == 0)
                .count()
                == BLOCK_SIZE
        }
    }

    pub fn check(&self) {
        if self.magic[0] != 'u' as u8
            || self.magic[1] != 's' as u8
            || self.magic[2] != 't' as u8
            || self.magic[3] != 'a' as u8
            || self.magic[4] != 'r' as u8
            || self.magic[5] != 32 as u8
        {
            panic!(
                "Invalid TAR Magic! (Expected: 'ustar\\0', found '{}', magic = {:?} (Size: {})",
                as_string(self.magic),
                self,
                core::mem::size_of::<PosixHeader>(),
            );
        }
    }
}

pub fn as_string<'retval, const N: usize>(slice: [u8; N]) -> ArrayString<N> {
    //let mut string = String::new();
    //slice
    //    .iter()
    //    .filter(|x| **x != 0)
    //    .for_each(|c| string.push(*c as char));
    let mut string = arrayvec::ArrayString::<N>::new();
    slice
        .iter()
        .filter(|x| **x != 0)
        .for_each(|c| string.push(*c as char));
    string
}

pub fn octal_ascii_size_as_usize(size: [u8; 12]) -> usize {
    usize::from_str_radix(as_string(size).as_str(), 8).unwrap()
}
