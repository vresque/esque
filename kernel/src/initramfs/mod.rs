use core::mem::MaybeUninit;

use alloc::vec::{self, Vec};
use bks::Handover;
use esqdrv::filesystem::{File, FileInfo, FileSystem, FileSystemObject};
use spin::Mutex;
use tar::tar::*;

use crate::{
    debug,
    heap::{malloc_mut, malloc_ptr},
    kprint,
    memory::paging::{
        page_frame_allocator::request_page, page_table_manager::GLOBAL_PAGE_TABLE_MANAGER,
    },
    scheduler::pit::{msleep, nanosleep, sleep},
    success,
};

pub static INITRAMFS: Mutex<MaybeUninit<FileSystemObject>> = Mutex::new(MaybeUninit::uninit());

pub fn load_initramfs(handover: &mut Handover) {
    let ptr = handover.initramfs_base;
    unsafe {
        GLOBAL_PAGE_TABLE_MANAGER
            .lock()
            .assume_init_mut()
            .map_memory(ptr, ptr);
    }
    let slice = unsafe { core::slice::from_raw_parts(ptr as *mut u8, handover.initramfs_size) };
    if ptr as *mut u64 == core::ptr::null_mut() || unsafe { *(ptr as *mut u64) } == 175 {
        panic!("InitRamFs is Null!");
    }

    let tar = Tar::from_slice(slice);
    let entries = tar.iter();
    for ent in entries {
        debug!("{}", ent.filename);
    }
    let buf = malloc_mut();
    *buf = tar;

    INITRAMFS.lock().write(InitRamFs::to_object(
        buf as *mut Tar as *mut u64 as u64,
        0,
        0,
    ));
}

pub struct InitRamFs;

unsafe impl FileSystem for InitRamFs {
    fn open_fs() {
        debug!("Opened InitRamFs")
    }

    fn close_fs(me: *mut FileSystemObject) {
        debug!("Closing InitRamFs")
    }

    fn open(me: *mut FileSystemObject, path: *mut u8) -> *mut File {
        unsafe {
            let tarf = &mut *((*me).buf1 as *mut Tar);
            let mut ent_orig: TarEntry = TarEntry::empty();
            let name_slice = core::slice::from_raw_parts_mut(path, 100 /* Max Len */);
            let str = tar::as_array_string(name_slice);
            for ent in tarf.iter() {
                if ent.filename == str {
                    ent_orig = ent;
                }
            }
            let tarent = malloc_mut();
            *tarent = ent_orig;
            if *tarent == TarEntry::empty() {
                panic!("Tried to open {:?}, but no matching file was found", str);
            }
            let file = malloc_mut();
            let file_info = FileInfo {
                size: (tarent.size as u64),
            };
            *file = File {
                read: read_tarent,
                write: write_tarent,
                buf: (tarent as *mut TarEntry as *mut u64 as u64),
                info: file_info,
            };
            file
        }
    }

    fn close(me: *mut FileSystemObject, fd: *mut esqdrv::filesystem::File) {
        todo!()
    }

    fn info(
        me: *mut FileSystemObject,
        fd: *mut esqdrv::filesystem::File,
    ) -> *mut esqdrv::filesystem::FileInfo {
        unsafe {
            let info = malloc_mut();
            let file_info = &(*fd).info;
            *info = *file_info;
            info as *mut FileInfo
        }
    }

    fn read(me: *mut FileSystemObject, fd: *mut esqdrv::filesystem::File, buf: *mut u8, size: u64) {
        todo!()
    }

    fn write(
        me: *mut FileSystemObject,
        fd: *mut esqdrv::filesystem::File,
        buf: *mut u8,
        size: u64,
    ) {
        todo!()
    }
}

pub extern "C" fn write_tarent(_: *mut File, contents: *mut u8, size: u64) -> u64 {
    size
}

pub extern "C" fn read_tarent(fd: *const File, buf: *mut u8, size: u64) {
    unsafe {
        let tarent = &mut *((*fd).buf as *mut u64 as *mut TarEntry);
        let slice_a = core::slice::from_raw_parts_mut(buf, size as usize);
        let slice_b = tarent.data;

        for i in 0..size {
            slice_a[i as usize] = slice_b[i as usize]
        }
    }
}
