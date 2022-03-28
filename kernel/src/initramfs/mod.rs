use core::mem::MaybeUninit;

use alloc::vec::Vec;
use bks::Handover;
use spin::Mutex;
use tar::tar::*;

use crate::{config::handover, memory::paging::page_table_manager::PAGE_TABLE_MANAGER};

pub static INITRAMFS: Mutex<MaybeUninit<InitRamFs>> = Mutex::new(MaybeUninit::uninit());

pub mod fs {
    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };

    use super::INITRAMFS;

    pub fn read_to_string(path: &str) -> Option<String> {
        Some(
            String::from_utf8_lossy(
                unsafe { INITRAMFS.lock().assume_init_mut() }
                    .open(path)?
                    .data,
            )
            .to_string(),
        )
    }

    pub fn read_until_end(path: &str) -> Option<Vec<u8>> {
        unsafe { Some(INITRAMFS.lock().assume_init_mut().open(path)?.data.to_vec()) }
    }
}

pub fn load_initramfs() {
    let mut handover = handover();
    let ptr = handover.initramfs_base;
    unsafe {
        PAGE_TABLE_MANAGER
            .lock()
            .assume_init_mut()
            .map_memory(ptr, ptr);
    }
    let slice = unsafe { core::slice::from_raw_parts(ptr as *mut u8, handover.initramfs_size) };
    if ptr as *mut u64 == core::ptr::null_mut() || unsafe { *(ptr as *mut u64) } == 175 {
        panic!("InitRamFs is Null!");
    }

    let tar = Tar::from_slice(slice);
    INITRAMFS.lock().write(InitRamFs::new(tar));
}

pub struct InitRamFs<'tar> {
    tar: Tar<'tar>,
}

impl<'tar> InitRamFs<'tar> {
    pub fn new(tar: Tar<'tar>) -> Self {
        Self { tar }
    }

    pub fn open(&self, path: &str) -> Option<TarEntry> {
        for ent in self.tar.iter() {
            if ent.filename == ArrayString::<100>::from(path).unwrap() {
                return Some(ent);
            }
        }
        None
    }

    // ALLOC: Alloc must be used here as there may be many .system files in the initramfs
    pub fn all_entries_with_extension(&self, ext: &str) -> Vec<TarEntry> {
        let mut ret: Vec<TarEntry> = alloc::vec![];
        for ent in self.tar.iter() {
            if ent.filename.ends_with(ext) {
                ret.push(ent)
            }
        }

        ret
    }

    pub fn entries(&self) -> TarIter {
        self.tar.iter()
    }
}

pub fn load_system_space_applications() {
    unsafe {
        let _sys_files = INITRAMFS
            .lock()
            .assume_init_mut()
            .all_entries_with_extension(".system");
    }
}
