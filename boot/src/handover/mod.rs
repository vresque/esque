use core::ffi::CStr;

use bks::Framebuffer;
use bks::Psf1Font;
use bks::Psf1Header;
use bks::PAGE_SIZE;
use log::error;
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::file::File;
use uefi::proto::media::file::FileInfo;
use uefi::table::boot::AllocateType;
use uefi::table::boot::MemoryType;
use uefi::table::boot::OpenProtocolAttributes;
use uefi::table::boot::OpenProtocolParams;
use uefi::table::Runtime;
use uefi::CStr16;
use uefi::Handle;

use crate::load_file;

const PSF1_MAGIC0: u8 = 0x36;
const PSF1_MAGIC1: u8 = 0x04;

pub fn init_gop(handle: Handle, table: &SystemTable<Boot>) -> Framebuffer {
    let gop_handle = table
        .boot_services()
        .get_handle_for_protocol::<GraphicsOutput>()
        .unwrap();
    info!("L, {:?}", gop_handle);
    let mut gop = table
        .boot_services()
        .open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .unwrap();
    info!("D");
    Framebuffer::new(
        gop.frame_buffer().as_mut_ptr() as u64,
        gop.frame_buffer().size(),
        gop.current_mode_info().resolution().0,
        gop.current_mode_info().resolution().1,
        gop.current_mode_info().stride(),
    )
}

pub fn create_font(handle: Handle, table: &SystemTable<Boot>) -> Option<Psf1Font> {
    let mut strbuf = [0; 9];
    let file = &mut load_file(
        None,
        CStr16::from_str_with_buf("font.psf", &mut strbuf).unwrap(),
        handle,
        table,
    )
    .unwrap();

    let ptr = table
        .boot_services()
        .allocate_pool(MemoryType::LOADER_DATA, core::mem::size_of::<Psf1Header>())
        .expect("Failed to allocate memory for font header");

    let size = core::mem::size_of::<Psf1Header>();
    let buffer = unsafe { core::slice::from_raw_parts_mut(ptr, size) };
    file.read(buffer).expect("Failed to read data into buffer");

    let header: Psf1Header = unsafe { core::ptr::read(buffer.as_ptr() as *const _) };

    info!("{:#?}", header);
    // Verify header
    if header.magic[0] != PSF1_MAGIC0 || header.magic[1] != PSF1_MAGIC1 {
        error!("Bad magic!");
        return None;
    }

    let buffer_size: usize = if header.mode == 1 {
        // 512 glyph mode
        ((header.charsize) as usize * 512) as usize
    } else {
        // 256 glyph mode
        ((header.charsize) as usize * 256) as usize
    };
    file.set_position(core::mem::size_of::<Psf1Header>() as u64)
        .expect("Failed to set font position");

    info!("Loading font into memory...");
    let buffer_ptr = table
        .boot_services()
        .allocate_pool(MemoryType::LOADER_DATA, buffer_size)
        .expect("Failed to allocate memory for font header");

    info!("Creating glyph buffer...");
    let glyph_buffer = unsafe { core::slice::from_raw_parts_mut(buffer_ptr, buffer_size) };
    file.read(glyph_buffer)
        .expect("Failed to read data into buffer");
    let font = Psf1Font::new(header, glyph_buffer.as_mut_ptr(), buffer_size as usize);
    info!("Finished creating font");
    Some(font)
}

pub fn read_initramfs(handle: Handle, table: &SystemTable<Boot>) -> Option<(u64, usize)> {
    let mut strbuf = [0; 14];
    let initramfs_file = &mut load_file(
        None,
        CStr16::from_str_with_buf("initramfs.tar", &mut strbuf).unwrap(),
        handle,
        table,
    )
    .unwrap();

    let mut info_buf: [u8; 512] = [0; 512];
    let info = initramfs_file
        .get_info::<FileInfo>(&mut info_buf)
        .expect("Failed to load InitRamFs File Info");
    info!("InitRamFs File Size: {}", info.file_size());
    let size = info.file_size() as usize;

    let pages = (size + PAGE_SIZE as usize - 1) / PAGE_SIZE as usize;
    let ptr = table
        .boot_services()
        .allocate_pages(
            AllocateType::MaxAddress(0x200000),
            MemoryType::LOADER_DATA,
            pages,
        )
        .expect("Failed to allocate for the InitRamFs");
    let file = unsafe { core::slice::from_raw_parts_mut(ptr as *mut u8, size) };

    // Reads all contents of the file into a buffer
    let read = initramfs_file
        .read(file)
        .expect("Failed to load file into buffer");
    // read == the bytes that were read (aka size). If not true, nothing was read
    assert_eq!(read, size);
    info!("{} -> {}", read, size);

    //info!("{:?}", file);
    info!("{:#x?}", ptr);
    Some((ptr, size))
}

pub fn find_rsdp(table: &SystemTable<Runtime>) -> u64 {
    let mut config = table.config_table().iter();

    let rsdp = config
        .find(|ent| matches!(ent.guid, uefi::table::cfg::ACPI2_GUID))
        .map(|ent| ent.address)
        .expect("An ACPI2 Compatible System is expected");

    rsdp as u64
}
