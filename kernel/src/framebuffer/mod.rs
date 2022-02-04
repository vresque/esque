use core::{fmt::Write, mem::MaybeUninit};

use spin::Mutex;

use bks::{Framebuffer, Psf1Font};
extern crate compiler_builtins;

// DISCUSS: Should Option be used here?
pub static FRAMEBUFFER_GUARD: Mutex<MaybeUninit<FramebufferGuard>> =
    Mutex::new(MaybeUninit::uninit());

#[repr(u32)]
// Not all colours are constructed
#[allow(dead_code)]
pub enum Color {
    Black = 0x00000000,
    White = 0xffffffff,
    Red = 0xff0000,
    Orange = 0xf5a905,
    Yellow = 0xf5f505,
    LightGreen = 0x84f505,
    DarkGreen = 0x4b6430,
    Cyan = 0x306450,
    LightBlue = 0x0bc1e2,
    DarkBlue = 0x0b3ce2,
    Purple = 0x630be2,
    Pink = 0xdb0be2,
    // TODO: Custom Variant Custom(u8, u8, u8)
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy)]
pub struct FramebufferGuard {
    framebuffer: Framebuffer,
    pub framebuffer_buffer: u32,
    font: Psf1Font,
    col: usize,
    row: usize,
    background: u32,
    foreground: u32,
    column_starting_point: usize,
}

impl FramebufferGuard {
    pub fn new(
        mut framebuffer: Framebuffer,
        font: Psf1Font,
        background: Color,
        foreground: Color,
    ) -> Self {
        Self {
            framebuffer_buffer: framebuffer.raw_buffer() as *mut u32 as u32,
            framebuffer: framebuffer,
            font: font,
            row: 0,
            col: 0,
            background: background as u32,
            foreground: foreground as u32,
            column_starting_point: 0,
        }
    }

    pub fn resolution(&mut self) -> (usize, usize, usize) {
        (
            self.framebuffer.width,
            self.framebuffer.height,
            self.framebuffer.stride,
        )
    }

    pub fn framebuffer(&mut self) -> &Framebuffer {
        &self.framebuffer
    }

    pub unsafe fn test(&mut self) {}
    pub unsafe fn clear_color<T>(&mut self, color: T)
    where
        T: Into<u32>,
    {
        let base = self.framebuffer_buffer as u64;
        let bytes_per_line = self.framebuffer.stride * 4;
        let color_as_u32: u32 = color.into();

        for vertical in 0..self.framebuffer.height {
            let pix_ptr_base = base + (vertical as u64 * bytes_per_line as u64);
            let mut pix_ptr = pix_ptr_base as *mut u32;
            *pix_ptr = 0xff;
            while pix_ptr < ((pix_ptr_base + bytes_per_line as u64) as *mut u32) {
                *pix_ptr = color_as_u32;
                pix_ptr = pix_ptr.add(1);
            }
        }
        self.set_location(0, 0);
    }

    pub fn set_column_starting_point(&mut self, new: usize) {
        self.column_starting_point = new;
    }

    pub fn set_color<T, U>(&mut self, background: T, foreground: U)
    where
        T: Into<u32>,
        U: Into<u32>,
    {
        self.foreground = foreground.into();
        self.background = background.into();
    }

    pub fn get_color(&mut self) -> (u32, u32) {
        (self.background, self.foreground)
    }

    pub fn set_location(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn new_line(&mut self) {
        self.col = self.column_starting_point;
        self.row += 16;
        self.new_line_checks();
    }

    fn new_line_checks(&mut self) {
        if self.row + 16 >= self.framebuffer.height - (16 * 2) {
            let top_row_max = self.framebuffer.stride * 4 * 16;

            for i in 0..top_row_max {
                unsafe {
                    *(self.framebuffer_buffer as *mut u8).add(i) = 0x00;
                }
            }

            unsafe {
                //let base = self.framebuffer_buffer as u64 + top_row_max as u64;
                //let size = self.framebuffer().size;
                //let slice_a: &mut [u8] = core::slice::from_raw_parts_mut(base as *mut u8, size);
                //let slice_b: &mut [u8] = core::slice::from_raw_parts_mut(
                //    self.framebuffer_buffer as *mut u8,
                //    self.framebuffer().size,
                //);
                ////for (old, new) in slice_a.iter().zip(slice_b) {
                //    *new = *old;
                //}
                // TEMP: Remove FIXME
                self.clear_color(self.background);
                self.row = 1;
                self.col = 0;

                // TODO: core::ptr::copy(base as *mut u8, self.framebuffer_buffer as *mut u8, size);
            }
            self.row -= 1;
        }
    }

    // DISCUSS: Should printing be considered unsafe?
    // In theory, we can ensure that nothing goes wrong, in practice that cannot be asserted
    pub unsafe fn print(&mut self, str: &str) {
        for c in str.chars() {
            match c {
                c if c.is_ascii() => self.draw_char(c),
                _ => {}
            }
        }
    }
    unsafe fn draw_char(&mut self, c: char) {
        match c {
            '\n' | '\r' => {
                self.new_line();
            }
            '\t' => {
                for _ in 0..12 {
                    self.put_char(' ');
                }
            }
            _ => {
                self.put_char(c);
                if self.framebuffer().width == self.col {
                    self.new_line();
                } else {
                    self.col += 8;
                }
            }
        }
    }

    pub fn clear_last_char(&mut self) {
        // Check that we do not clear nonexistant screen space
        if self.col == 0 {
            self.col = self.framebuffer().width;
            if (self.row as isize) - 16 < 0 {
                self.row = 0;
            } else {
                self.row -= 16;
            }
        }

        let charsize = self.font.header().charsize as usize;
        let stride = self.framebuffer.stride;

        for y in self.row..(self.row + 16) {
            for x in (self.col - 8)..(self.col) {
                let offset = x + (y * stride);
                unsafe {
                    let ptr = (self.framebuffer_buffer as *mut u32).add(offset);
                    *ptr = self.background;
                }
            }
        }
        if (self.col as isize) - 8 < 0 {
            self.col = self.framebuffer().width;
            if (self.row as isize) - 16 < 0 {
                self.row = 0;
            } else {
                self.row -= 16;
            }
        } else {
            self.col -= 8;
        }
    }

    unsafe fn put_char(&mut self, chr: char) {
        let charsize = self.font.header().charsize as usize;
        let stride = self.framebuffer.stride;
        let font_offset = (chr as u8 as usize * charsize) as usize;
        let mut font_ptr = (self.font.buffer as *mut u8).add(font_offset);

        for y in self.row..(self.row + 16) {
            for x in self.col..(self.col + 8) {
                let offset = x + (y * stride);
                let ptr = (self.framebuffer_buffer as *mut u32).add(offset);
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

pub fn clear_screen<T>(color: T)
where
    T: Into<u32>,
{
    unsafe {
        FRAMEBUFFER_GUARD
            .lock()
            .assume_init_mut()
            .clear_color(color);
    }
}

impl Write for FramebufferGuard {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        unsafe {
            self.draw_char(c);
        };
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            self.print(s);
        };

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////
/// ------------------------------------ Macros ----------------------------------------
////////////////////////////////////////////////////////////////////////////////////////
#[macro_export]
macro_rules! kprintln {
    () => ({
        use crate::framebuffer::FRAMEBUFFER_GUARD;
        use core::fmt::Write;

        unsafe { FRAMEBUFFER_GUARD.lock().assume_init_mut().write_str("\n").unwrap(); };
    });
    ($($arg:tt)*) => ({
        use crate::framebuffer::FRAMEBUFFER_GUARD;
        use core::fmt::Write;

        unsafe { FRAMEBUFFER_GUARD.lock().assume_init_mut().write_fmt(format_args_nl!($($arg)*)).unwrap(); }
    })
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        use crate::framebuffer::FRAMEBUFFER_GUARD;
        use core::fmt::Write;

        unsafe { FRAMEBUFFER_GUARD.lock().assume_init_mut().write_fmt(format_args!($($arg)*)).unwrap(); };
    })
}

#[macro_export]
macro_rules! emergency {
    ($($arg:tt)*) => ({
        use crate::{kprint, kprintln};
        use crate::kscopedcolorchange;
        use crate::framebuffer::Color;
        kscopedcolorchange!(bg: Color::White, fg: Color::Red => {
            kprint!("[ EMERGENCY ][{}:{}:{}] -> ", file!(), line!(), column!());
        });
        kscopedcolorchange!(bg: Color::White, fg: Color::Black => {
            kprintln!($($arg)*);
        });
    })
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        use crate::{kprint, kprintln};
        use crate::kscopedcolorchange;
        use crate::framebuffer::Color;
        kscopedcolorchange!(bg: Color::Black, fg: Color::Orange => {
            kprint!("[ WARNING ][{}:{}:{}] -> ", file!(), line!(), column!());
        });
        kprintln!($($arg)*);
    })
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        use crate::{kprint, kprintln};
        use crate::kscopedcolorchange;
        use crate::framebuffer::Color;
        kscopedcolorchange!(bg: Color::Black, fg: Color::Red => {
            kprint!("[ ERROR ][{}:{}:{}] -> ", file!(), line!(), column!());
        });
        kprintln!($($arg)*);
    })
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => ({
        use crate::{kprint, kprintln};
        use crate::kscopedcolorchange;
        use crate::framebuffer::Color;
        kscopedcolorchange!(bg: Color::White, fg: Color::DarkGreen => {
            kprint!("[ SUCCESS ][{}:{}:{}] -> ", file!(), line!(), column!());
        });
        kscopedcolorchange!(bg: Color::White, fg: Color::Black => {
            kprintln!($($arg)*);
        });
    })
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        use crate::{kprint, kprintln};
        kprint!("[ INFO ][{}:{}:{}] -> ", file!(), line!(), column!());
        kprintln!($($arg)*);
    })
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        use crate::{kprint, kprintln};
        use crate::kscopedcolorchange;
        use crate::framebuffer::Color;
        kscopedcolorchange!(bg: Color::Cyan, fg: Color::Yellow => {
            kprint!("[ DEBUG ][{}:{}:{}] -> ", file!(), line!(), column!());
        });
        kscopedcolorchange!(bg: Color::Cyan, fg: Color::White => {
            kprintln!($($arg)*);
        })
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{}};
}

#[macro_export]
macro_rules! kcolorchange {
    (bg: $bg:expr, fg: $fg:expr) => {{
        use crate::framebuffer::FRAMEBUFFER_GUARD;
        unsafe {
            FRAMEBUFFER_GUARD
                .lock()
                .assume_init_mut()
                .set_color($bg, $fg);
        }
    }};
}

#[macro_export]
macro_rules! kscopedcolorchange {
    (bg: $bg:expr, fg: $fg:expr => $blck:block) => {{
        use crate::framebuffer::FRAMEBUFFER_GUARD;
        use crate::kcolorchange;
        let old = unsafe {
            FRAMEBUFFER_GUARD.lock().assume_init_mut().get_color()
        };
        kcolorchange!(bg: $bg, fg: $fg);
        {
            $blck
        }
        kcolorchange!(bg: old.0, fg: old.1);
    }};
}
