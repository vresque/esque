use core::marker::PhantomData;

use alloc::string::String;
use bks::PAGE_SIZE;
use esqdrv::filesystem::FileSystemObject;
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
                protected,
            }
        }
    }

    pub fn with_pid<'me>(mut self, pid: Pid) -> Self {
        self.pid = pid;
        return self;
    }

    pub fn launch(self) -> Process {
        unsafe {
            FRAMEBUFFER_GUARD
                .lock()
                .assume_init_mut()
                .clear_color(0x0u32);
        };
        success!("Loading Elf...");
        let elf = ElfFile::new(self.data).expect("initfs has an invalid format");
        header::sanity_check(&elf).unwrap();
        let mut page_sum = 0;
        success!("Loading Elf...");

        // We iterate over the Program Headers twice
        // Once, to get the amount of memory we need
        for header in elf.program_iter() {
            program::sanity_check(header, &elf);
            if let ProgramHeader::Ph64(hdr) = header {
                match hdr.get_type().unwrap() {
                    program::Type::Load => page_sum += hdr.physical_addr,
                    _ => {}
                }
            } else {
                panic!("Found invalid Program Header")
            }
        }
        // let start = unsafe {
        //     PAGE_FRAME_ALLOCATOR
        //         .lock()
        //         .assume_init_mut()
        //         .find_first_consecutive_free_pages_of_count(page_sum as usize)
        // };
        let my_malloced_ptr = unsafe { malloc_ptr::<u8>(page_sum as usize) };
        let slice = unsafe { core::slice::from_raw_parts_mut(my_malloced_ptr, page_sum as usize) };
        for header in elf.program_iter() {
            if let ProgramHeader::Ph64(hdr) = header {
                match hdr.get_type().unwrap() {
                    program::Type::Load => {
                        let pages = hdr.mem_size + (PAGE_SIZE - 1) / PAGE_SIZE;
                        let segment = hdr.physical_addr;
                        // unsafe {
                        //     if PAGE_FRAME_ALLOCATOR
                        //         .lock()
                        //         .assume_init_mut()
                        //         .allocate_from_addr_to_count_unchecked(segment, pages as usize)
                        //         == false
                        //     {
                        //         panic!(
                        //             "Failed to allocate while allocating for header {:?}",
                        //             header
                        //         );
                        //     };
                        // };

                        let data = match hdr
                            .get_data(&elf)
                            .expect("Failed to read SegmentHeader data")
                        {
                            SegmentData::Undefined(data) => data,
                            data => {
                                panic!("The DataType {:?} is not yet implemented", data);
                            }
                        };
                        //let mut where_to =
                        //    &slice[(segment as usize)..(segment as usize + hdr.mem_size as usize)];
                        //where_to = data;
                        unsafe {
                            core::ptr::copy(
                                data.as_ptr(),
                                my_malloced_ptr.add(segment as usize),
                                data.len(),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        let entry = elf.header.pt2.entry_point() + my_malloced_ptr as u64;
        let function: fn(u64, u64) -> u64 =
            unsafe { core::mem::transmute(elf.header.pt2.entry_point() + my_malloced_ptr as u64) };
        success!("Successfully loaded ELF");
        success!("Function returned: {}", function(2, 3));

        Process::new(self.pid, entry as u64, 0, self.protected)
    }
}
