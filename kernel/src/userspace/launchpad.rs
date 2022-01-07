use core::marker::PhantomData;

use alloc::string::String;
use bks::PAGE_SIZE;
use esqdrv::device::FileSystemObject;
use tar::tar::Tar;
use xmas_elf::{
    header,
    program::{self, ProgramHeader, SegmentData},
    ElfFile,
};

use crate::{
    debug, error,
    framebuffer::FRAMEBUFFER_GUARD,
    heap::{malloc_mut, malloc_ptr},
    info,
    memory::{paging::page_frame_allocator::PAGE_FRAME_ALLOCATOR, userspace::LAST_VIRT_MEM},
    success,
    userspace::process::Process,
};

use super::pid::Pid;
#[derive(Copy, Clone)]
pub struct KernelSpace;
#[derive(Copy, Clone)]
pub struct Userspace;

#[derive(Debug, Copy, Clone)]
pub struct Launchpad<'data, T> {
    data: &'data [u8],
    pid: Pid,
    _phantom: PhantomData<T>,
    protected: bool,
}

impl<'data> Launchpad<'data, KernelSpace> {
    pub fn new<'fs>(fs: FileSystemObject, path: &'data str, protected: bool) -> Self {
    }

    pub fn with_pid<'me>(mut self, pid: Pid) -> Self {
        self.pid = pid;
        return self;
    }

    pub fn launch(self) -> Process {
    }
}
