use core::{fmt::Write, cell::UnsafeCell};

use spin::Mutex;

use bks::{Framebuffer, Handover, Psf1Font};
use lazy_static::lazy_static;

pub static mut FRAMEBUFFER_WRITER: Option<FramebufferWriter> = None;

#[repr(u32)]
pub enum Color {
    Black       = 0x00000000,
    White       = 0xffffffff,
    Red         = 0xf72d02,
    Orange      = 0xf5a905,
    Yellow      = 0xf5f505,
    LightGreen  = 0x84f505,
    DarkGreen   = 0x4b6430,
    Cyan        = 0x306450,
    LightBlue   = 0x0bc1e2,
    DarkBlue    = 0x0b3ce2,
    Purple      = 0x630be2,
    Pink        = 0xdb0be2,
    // TODO: Custom Variant Custom(u8, u8, u8)
}

pub struct FramebufferWriter<'a> {
    framebuffer: &'a mut Framebuffer,
    framebuffer_buffer: *mut u32,
    font: &'a mut Psf1Font,
    col: usize,
    row: usize,
    background: u32,
    foreground: u32,
}

impl<'a> FramebufferWriter<'a> {
    pub fn new(handover: &'a mut Handover, background: Color, foreground: Color) -> Self {
        Self {
            framebuffer_buffer: handover.framebuffer().raw_buffer() as *mut u32,
            framebuffer: &mut handover.framebuffer(),
            font: &mut handover.font(),
            row: 20,
            col: 20,
            background: background as u32,
            foreground: foreground as u32,
        }
    }

    pub unsafe fn print(&mut self, str: &str) {
        for c in str.chars() {
            match c {
                '\n' => {
                    self.col = 0;
                    self.row += 16;
                },
                '\t' => {
                    for i in 0..4 {
                        self.draw_char(' ');
                    }
                },
                _ => {
                    self.draw_char(c);
                }
            }

            self.col += 8;
            if self.col + 8 > self.framebuffer.width {
                self.col = 0;
                self.row += 16;
            }
        }
    }

    unsafe fn draw_char(
        &mut self,
        chr: char,
    ) {
        let charsize = self.font.header().charsize as usize;
        let stride = self.framebuffer.stride;
        let font_offset = (chr as u8 as usize * charsize) as usize;
        let mut font_ptr = self.font.buffer.add(font_offset);
        let mut index = 0;
    
        for y in self.row..(self.row + 16) {
            for x in self.col..(self.col + 8) {
                let offset = x + (y * stride);
                let ptr = self.framebuffer_buffer.add(offset);
                if (*font_ptr & (0b10000000 >> (x - self.col))) > 0 {
                    *ptr = self.foreground;
                } else {
                    *ptr = self.background;
                }
            }
            font_ptr = font_ptr.add(1);
        }
    }
}

impl<'a> Write for FramebufferWriter<'a> {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        unsafe { self.draw_char(c); };
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { self.print(s); };
        Ok(())
    }
}
