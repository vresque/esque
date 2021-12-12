#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

extern crate alloc;
extern crate uefi;
extern crate uefi_services;

mod handover;
use core::mem::size_of;

use crate::alloc::vec::Vec;
use alloc::vec;
use bks::{EfiMemoryDescriptor, Handover};
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
    table::boot::{AllocateType, MemoryDescriptor, MemoryType},
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
    table: &SystemTable<Boot>,
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
        Some(d) => d,
        None => filesystem
            .open_volume()
            .expect_success("Failed to open root volume"),
    };

    let filehandle = match directory.open(path, FileMode::Read, FileAttribute::READ_ONLY) {
        Ok(fh) => fh.unwrap(),
        Err(e) => {
            error!("Failed to open file '{}'\nError: {:?}", path, e);
            return Err(Status::NOT_FOUND);
        }
    };

    Ok(unsafe { RegularFile::new(filehandle) })
}

pub fn load_kernel(mut kfile: RegularFile, table: &SystemTable<Boot>) -> Result<u64, Status> {
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
    let kernel = match load_file(None, "esque", handle, &mut table) {
        Ok(k) => k,
        Err(e) => {
            error!("Failed to find kernel!\n Error: {:?}", e);
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
    // Exiting the boot services is required to get the memory map (At least in this library)
    //let (rt_table, mut handover) = create_handover_and_exit_boot_services(handle, table);

    let mmap_size: usize = table.boot_services().memory_map_size();
    let buf_size: usize = mmap_size + 8 * size_of::<MemoryDescriptor>();
    let buffer = vec![0_u8; buf_size];
    info!("{}", buffer.len());

    let descriptors: &mut [EfiMemoryDescriptor; 255] = &mut [EfiMemoryDescriptor::empty(); 255];

    let mmap_sz = table.boot_services().memory_map_size();
    let mmap_storage = {
        let buf_sz = mmap_sz + 8 * size_of::<MemoryDescriptor>();

        let ptr = table
            .boot_services()
            .allocate_pool(MemoryType::LOADER_DATA, buf_sz)
            .expect_success("Failed to allocate memory for Memory Map");
        unsafe { core::slice::from_raw_parts_mut(ptr, buf_sz) }
    };

    let framebuffer = handover::init_gop(handle, &mut table);
    info!("{}", framebuffer);
    let font = match handover::create_font(handle, &mut table) {
        Some(t) => t,
        None => panic!("Failed to find font"),
    };

    let (_rt_table, memory_map) = table
        .exit_boot_services(handle, mmap_storage)
        .expect_success("Failed to exit boot services");

    descriptors
        .iter_mut()
        .zip(
            memory_map.map(|m| unsafe {
                core::mem::transmute::<MemoryDescriptor, EfiMemoryDescriptor>(*m)
            }),
        )
        .fold(0, |count, (dest, item)| {
            *dest = item;
            count + 1
        });

    let mut entries = 0;
    descriptors.iter_mut().fold(0, |count, e| {
        if e != &mut EfiMemoryDescriptor::empty() {
            entries += 1;
        }
        count + 1
    });

    // I am not sure about this
    // But, as the kernel uses it as mut, I do not wish
    // that this is ever placed into readonly-memory
    #[allow(unused_mut)]
    let mut handover = Handover::new(
        framebuffer,
        font,
        descriptors.as_mut_ptr(),
        mmap_size,
        entries,
    );

    kmain(handover);
    Status::SUCCESS
}
