use core::str::Utf8Error;

use alloc::string::String;

use crate::{
    header::{as_string, octal_ascii_size_as_usize, PosixHeader, BLOCK_SIZE},
    types::TarEntryType,
};

pub struct TarEntry<'data> {
    pub filename: String,
    pub data: &'data [u8],
    pub size: usize,
    pub ty: u8,
}

impl<'data> TarEntry<'data> {
    pub const fn new(fname: String, data: &'data [u8], ty: u8) -> Self {
        Self {
            filename: fname,
            data: data,
            size: data.len(),
            ty: ty,
        }
    }

    pub fn to_utf8_string(&self) -> Result<&'data str, Utf8Error> {
        core::str::from_utf8(self.data)
    }
}

pub struct Tar<'data> {
    data: &'data [u8],
}

impl<'data> Tar<'data> {
    pub const fn from_slice(bytes: &'data [u8]) -> Self {
        Self { data: bytes }
    }

    pub fn iter(&self) -> TarIter<'data> {
        TarIter::genesis(self.data)
    }
}

pub struct TarIter<'data> {
    data: &'data [u8],
    idx: usize,
}

impl<'data> Iterator for TarIter<'data> {
    type Item = TarEntry<'data>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx * BLOCK_SIZE >= self.data.len() {
            return None;
        }

        let block_header = self.next_block_header();
        block_header.check();

        if block_header.is_null() {
            // EOF?
            return None;
        }

        // Blocks required for data
        let block_count = block_header.block_count();
        let begin = (self.idx + 1) * BLOCK_SIZE;
        let end = begin + block_count * BLOCK_SIZE;

        let bytes = (&self.data[begin..end][0..octal_ascii_size_as_usize(block_header.size)]);

        let entry = TarEntry::new(
            as_string(block_header.name),
            bytes,
            TarEntryType::RegularFile,
        );
        Some(entry)
    }
}

impl<'block> TarIter<'block> {
    pub const fn genesis(archive: &'block [u8]) -> Self {
        Self {
            data: archive,
            idx: 0,
        }
    }

    pub fn next_block_header(&self) -> &'block PosixHeader {
        self.block_header_at(self.idx)
    }

    pub fn block_header_at(&self, idx: usize) -> &'block PosixHeader {
        unsafe {
            let ptr = (&self.data[idx * BLOCK_SIZE as usize]) as *const u8 as *mut PosixHeader;
            ptr.as_ref().unwrap()
        }
    }
}
