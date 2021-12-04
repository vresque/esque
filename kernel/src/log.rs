use core::{fmt::Write, cell::UnsafeCell, mem::MaybeUninit};

use spin::Mutex;

use bks::{Framebuffer, Handover, Psf1Font};
use lazy_static::lazy_static;

pub static mut FRAMEBUFFER_GUARD: MaybeUninit<FramebufferGuard> = MaybeUninit::uninit();

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

pub struct FramebufferGuard {
    framebuffer: Framebuffer,
    framebuffer_buffer: *mut u32,
    font: Psf1Font,
    col: usize,
    row: usize,
    background: u32,
    foreground: u32,
}

impl FramebufferGuard {
    pub fn new(mut framebuffer: Framebuffer, mut font: Psf1Font, background: Color, foreground: Color) -> Self {
        Self {
            framebuffer_buffer: framebuffer.raw_buffer() as *mut u32,
            framebuffer: framebuffer,
            font: font,
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

#[macro_export]
macro_rules! kprintln {
    () => {
        use crate::log::FRAMEBUFFER_GUARD;
        FRAMEBUFFER_GUARD.assume_init_mut().print("\n");
    };
    ($($arg:tt)*) => ({
        use crate::log::FRAMEBUFFER_GUARD;
        use core::fmt::Write;

        unsafe { FRAMEBUFFER_GUARD.assume_init_mut().write_fmt(format_args_nl!($($arg)*)); };
    })
}

impl Write for FramebufferGuard {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        unsafe { self.draw_char(c); };
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { self.print(s); };
        Ok(())
    }
}
