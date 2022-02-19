use core::{
    mem,
    ops::{Deref, DerefMut},
    slice,
};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Stat {
    pub dev: u64,
    pub ino: u64,
    pub mode: u16,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub blksize: u32,
    pub blocks: u64,
    pub mtime: u64,
    pub mtime_nsec: u32,
    pub atime: u64,
    pub atime_nsec: u32,
    pub ctime: u64,
    pub ctime_nsec: u32,
}

impl Deref for Stat {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Stat as *const u8, mem::size_of::<Stat>()) }
    }
}

impl DerefMut for Stat {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Stat as *mut u8, mem::size_of::<Stat>()) }
    }
}
