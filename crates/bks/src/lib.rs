#![no_std]

use core::slice;

pub struct Handover {
    // Must always be 42: If not, a bad bootloader was used
    checknum: u32,
    framebuffer: Framebuffer,
    font: Psf1Font,
}

impl Handover {
    pub fn new(fb: Framebuffer, font: Psf1Font) -> Self {
        Self {
            checknum: 42,
            framebuffer: fb,
            font: font,
        }
    }

    pub fn checknum(&self) -> u32 {
        self.checknum
    }

    pub fn framebuffer(&mut self) -> &mut Framebuffer {
        &mut self.framebuffer
    }

    pub fn font(&mut self) -> &mut Psf1Font {
        &mut self.font
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Psf1Header {
    pub magic: [u8; 2],
    pub mode: u8,
    pub charsize: u8,
}

impl Psf1Header {
    pub fn charsize(&mut self) -> u8 {
        self.charsize
    }
}

#[derive(Clone, Copy)]
pub struct Psf1Font {
    pub header: Psf1Header,
    pub buffer: *mut u8,
    pub size: usize,
}

impl Psf1Font {
    pub fn new(header: Psf1Header, buffer: *mut u8, size: usize) -> Self {
        Self {
            header,
            buffer,
            size,
        }
    }

    pub fn header(&mut self) -> &mut Psf1Header {
        &mut self.header
    }

    pub fn buffer(&self) -> &[u8] {
        unsafe { self.retrieve_buffer() }
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        unsafe { self.retrieve_buffer() }
    }

    unsafe fn retrieve_buffer<'a>(&self) -> &'a mut [u8] {
        slice::from_raw_parts_mut(self.buffer, self.size)
    }
}

#[derive(Clone, Copy)]
pub struct Framebuffer {
    base: *mut u8,
    size: usize,
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

impl Framebuffer {
    pub fn new(base: *mut u8, size: usize, width: usize, height: usize, stride: usize) -> Self {
        Self {
            base,
            size,
            width,
            height,
            stride,
        }
    }

    pub fn raw_buffer(&mut self) -> *mut u8 {
        self.base
    }

    pub fn buffer(&self) -> &[u8] {
        unsafe { self.retrieve_buffer() }
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        unsafe { self.retrieve_buffer() }
    }

    pub fn clear_bcolor(&mut self, color: u32) {
        for byte in self.buffer_mut() {
            // This upcast allows colours up to u32::MAX
            // Without this, a u8 will be used which only allows
            // 256 colours.
            unsafe { *(byte as *mut u8) = color as u8 };
        }
    }

    unsafe fn retrieve_buffer<'a>(&self) -> &'a mut [u8] {
        slice::from_raw_parts_mut(self.base, self.size)
    }
}

impl core::fmt::Display for Framebuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Framebuffer Information: ");
        writeln!(f, "   Base Address: {:#x?}", self.base);
        writeln!(f, "   Size: {:#x}", self.size);
        writeln!(f, "   Width: {}", self.width);
        writeln!(f, "   Height: {}", self.height);
        writeln!(f, "   Stride: {}", self.stride);
        Ok(())
    }
}
