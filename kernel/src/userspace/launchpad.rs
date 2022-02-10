use core::marker::PhantomData;

use super::pid::Pid;
#[derive(Copy, Clone)]
pub struct SystemSpace;
#[derive(Copy, Clone)]
pub struct Userspace;

#[derive(Debug, Copy, Clone)]
pub struct Launchpad<'data, T> {
    _data: &'data [u8],
    _pid: Pid,
    _phantom: PhantomData<T>,
    _protected: bool,
}

impl<'data> Launchpad<'data, SystemSpace> {
    //pub fn new<'fs>(fs: FileSystemObject, path: &'data str, protected: bool) {}

    pub fn with_pid<'me>(mut self, pid: Pid) -> Self {
        self._pid = pid;
        return self;
    }

    //pub fn launch(self) -> Process {}
}
