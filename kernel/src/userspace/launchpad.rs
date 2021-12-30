use core::marker::PhantomData;

use alloc::string::String;
use esqdrv::filesystem::FileSystemObject;
use tar::tar::Tar;

use crate::{
    debug,
    heap::{malloc_mut, malloc_ptr},
};

use super::pid::Pid;
pub struct KernelSpace;
pub struct Userspace;

pub struct Launchpad<'data, T> {
    data: &'data [u8],
    pid: Pid,
    _phantom: PhantomData<T>,
}

impl<'data> Launchpad<'data, KernelSpace> {
    pub fn new<'fs>(fs: FileSystemObject, path: &'data str) -> Self {
        let fspace = malloc_mut();
        *fspace = fs;
        unsafe {
            let str_ptr = malloc_ptr::<u8>(path.len());
            let slice = core::slice::from_raw_parts_mut(str_ptr, path.len());
            for i in 0..path.len() {
                slice[i as usize] = path.as_bytes()[i as usize];
            }

            let file = (fs.open)(fspace as *mut FileSystemObject, str_ptr);
            let file_size = (*(fs.info)(fspace as *mut FileSystemObject, file)).size;
            let buf = malloc_ptr(file_size as usize);
            ((*file).read)(file, buf, file_size);
            let data = core::slice::from_raw_parts_mut(buf, file_size as usize);
            Self {
                data,
                pid: (Pid::random()),
                _phantom: PhantomData,
            }
        }
    }

    pub fn with_pid(self, pid: Pid) -> Self {
        self.pid = pid;
        return self;
    }

    pub fn launch(self) {
        debug!("{:?}", core::str::from_utf8(self.data));
    }
}
