#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

extern crate alloc;
extern crate uefi;
extern crate uefi_services;

mod handover;
use crate::alloc::vec::Vec;
use alloc::vec;
use bks::Handover;
use handover::create_handover;
use log::{error, info};
use uefi::{
    prelude::*,
    proto::{
        loaded_image::LoadedImage,
        media::{
            file::{Directory, File, FileAttribute, FileInfo, FileMode, RegularFile},
            fs::SimpleFileSystem,
        },
    },
    table::boot::{AllocateType, MemoryType},
};
use xmas_elf::{
    header::sanity_check,
    program::{self, ProgramHeader, SegmentData},
    ElfFile,
};

pub fn load_file<'a>(
    dir: Option<Directory>,
    path: &str,
    handle: Handle,
    mut table: &SystemTable<Boot>,
) -> Result<RegularFile, Status> {
    let loaded_img = unsafe {
        &mut *(table
            .boot_services()
            .handle_protocol::<LoadedImage>(handle)
            .expect_success("Failed to load the LoadedImage Protocol")
            .get())
    };

    let filesystem = unsafe {
        &mut *(table
            .boot_services()
            .handle_protocol::<SimpleFileSystem>(loaded_img.device())
            .expect_success("Failed to open Root Filesystem")
            .get())
    };

    let mut directory = match dir {
        Some(mut d) => d,
        None => filesystem
            .open_volume()
            .expect_success("Failed to open root volume"),
    };

    let filehandle = match directory.open(path, FileMode::Read, FileAttribute::READ_ONLY) {
        Ok(fh) => fh.unwrap(),
        Err(e) => {
            error!("Failed to open file '{}'", path);
            return Err(Status::NOT_FOUND);
        }
    };

    Ok(unsafe { RegularFile::new(filehandle) })
}

pub fn load_kernel(mut kfile: RegularFile, mut table: &SystemTable<Boot>) -> Result<u64, Status> {
    let mut info_buf: [u8; 512] = [0; 512];
    let info = kfile
        .get_info::<FileInfo>(&mut info_buf)
        .expect_success("Failed to load Kernel File Info");
    info!("Kernel File Size: {}", info.file_size());
    let size = info.file_size() as usize;
    let mut file: Vec<u8> = vec![0; size];

    // Reads all contents of KFILE into buffer
    let read = kfile
        .read(file.as_mut_slice())
        .expect_success("Failed to load file into buffer");
    // read == the bytes that were read (aka size). If not true, nothing was read
    assert_eq!(read, size);

    let elf = ElfFile::new(file.as_slice()).expect("Failed to load Kernel Elf");
    sanity_check(&elf).expect("Failed to verify elf integrity");

    info!("Found entry point at {:#x?}", elf.header.pt2.entry_point());
    for phdr in elf.program_iter() {
        program::sanity_check(phdr, &elf).expect("Failed to verify program header integrity");
        // We only support 64 bits, therefore
        // All 32 bit headers can be discarded
        // Otherwise, all hdr calls match the header
        // Which can be quite resource intense
        if let ProgramHeader::Ph64(hdr) = phdr {
            match hdr.get_type().unwrap() {
                program::Type::Load => {
                    info!(
                        "Allocating for program header (at {:#x?})",
                        hdr.physical_addr
                    );
                    let pages = (hdr.mem_size + 0x1000 - 1) / 0x1000;
                    let segment = hdr.physical_addr as usize;
                    table
                        .boot_services()
                        .allocate_pages(
                            AllocateType::Address(segment),
                            MemoryType::LOADER_DATA,
                            pages as usize,
                        )
                        .expect_success("Failed to load Data into Memory");

                    let data = match hdr.get_data(&elf).expect("Failed to read phdr data") {
                        SegmentData::Undefined(u) => u,
                        a => {
                            error!("Found unhandable phdr data: {:#?}", a);
                            return Err(Status::UNSUPPORTED);
                        }
                    };

                    unsafe {
                        core::ptr::copy(data.as_ptr(), segment as *mut u8, data.len());
                    };
                }
                _ => {}
            }
        } else {
            error!("Found unsupported Program Header");
            return Err(Status::UNSUPPORTED);
        }
    }

    Ok(elf.header.pt2.entry_point())
}

#[entry]
fn efi_main(handle: uefi::Handle, mut table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut table).expect_success("Failed to setup Logging");

    table
        .stdout()
        .reset(false)
        .expect_success("Failed to reset output buffer");

    {
        let rev = table.uefi_revision();
        info!("Launching Gaia v{}.{}", rev.major(), rev.minor());
    }

    info!("Loading Kernel... (/esque)");
    let mut kernel = match load_file(None, "esque", handle, &mut table) {
        Ok(k) => k,
        Err(e) => {
            error!("Failed to find kernel!");
            return e;
        }
    };

    let entry = match load_kernel(kernel, &mut table) {
        Ok(e) => e,
        Err(e) => {
            error!("Failed to load kernel!");
            return e;
        }
    };
    let kmain: extern "sysv64" fn(info: Handover) -> u32 = unsafe { core::mem::transmute(entry) };
    let mut handover = create_handover(handle, &mut table);
    info!("Jumping to kernel...");
    info!("{}", kmain(handover));

    Status::SUCCESS
}
