#![no_std]

use core::slice;

pub struct Handover {
    // Must always be 42: If not, a bad bootloader was used
    checknum: u32,
    framebuffer: Framebuffer,
}

impl Handover {
    pub fn new(fb: Framebuffer) -> Self {
        Self {
            checknum: 42,
            framebuffer: fb,
        }
    }

    pub fn checknum(&self) -> u32 {
        self.checknum
    }

    pub fn framebuffer(&mut self) -> &mut Framebuffer {
        &mut self.framebuffer
    }
}

pub struct Framebuffer {
    base: *mut u8,
    size: usize,
    width: usize,
    height: usize,
    stride: usize,
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

    pub fn buffer(&self) -> &[u8] {
        unsafe { self.retrieve_buffer() }
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        unsafe { self.retrieve_buffer() }
    }

    pub fn clear_bcolor(&mut self, color: u8) {
        for byte in self.buffer_mut() {
            *byte = color;
        }
    }

    fn retrieve_buffer<'a>(&self) -> &'a mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.base, self.size) }
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
