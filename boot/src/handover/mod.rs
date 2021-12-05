use core::mem::size_of;
use core::mem::transmute;

use alloc::vec;
use alloc::vec::Vec;
use bks::EfiMemoryDescriptor;
use bks::Framebuffer;
use bks::Handover;
use bks::Psf1Font;
use bks::Psf1Header;
use log::error;
use log::info;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::file::File;
use uefi::table::boot::MemoryDescriptor;
use uefi::table::boot::MemoryMapKey;
use uefi::table::boot::MemoryType;
use uefi::table::Runtime;
use uefi::Handle;

use crate::load_file;

const PSF1_MAGIC0: u8 = 0x36;
const PSF1_MAGIC1: u8 = 0x04;

pub fn init_gop(handle: Handle, mut table: &SystemTable<Boot>) -> Framebuffer {
    let gop = unsafe {
        &mut *(table
            .boot_services()
            .locate_protocol::<GraphicsOutput>()
            .expect_success("Failed to locate GOP")
            .get())
    };

    Framebuffer::new(
        gop.frame_buffer().as_mut_ptr(),
        gop.frame_buffer().size(),
        gop.current_mode_info().resolution().0,
        gop.current_mode_info().resolution().1,
        gop.current_mode_info().stride(),
    )
}

pub fn create_font(handle: Handle, mut table: &SystemTable<Boot>) -> Option<Psf1Font> {
    let file = &mut load_file(None, "font.psf", handle, table).unwrap();

    let ptr = table
        .boot_services()
        .allocate_pool(MemoryType::LOADER_DATA, core::mem::size_of::<Psf1Header>())
        .expect_success("Failed to allocate memory for font header");

    let size = core::mem::size_of::<Psf1Header>();
    let buffer = unsafe { core::slice::from_raw_parts_mut(ptr, size) };
    file.read(buffer)
        .expect_success("Failed to read data into buffer");

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
        .expect_success("Failed to set font position");

    info!("Loading font into memory...");
    let buffer_ptr = table
        .boot_services()
        .allocate_pool(MemoryType::LOADER_DATA, buffer_size)
        .expect_success("Failed to allocate memory for font header");

    info!("Creating glyph buffer...");
    let glyph_buffer = unsafe { core::slice::from_raw_parts_mut(buffer_ptr, buffer_size) };
    file.read(glyph_buffer)
        .expect_success("Failed to read data into buffer");
    let font = Psf1Font::new(header, glyph_buffer.as_mut_ptr(), buffer_size as usize);
    Some(font)
}
