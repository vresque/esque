#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(asm)]

use core::{
    ops::{Shr, Sub},
    panic::PanicInfo,
};
mod init;
mod log;
use bks::{EfiMemoryDescriptor, Handover};
use init::common::init_common;
use log::Color;

use crate::log::FRAMEBUFFER_GUARD;

static mut COUNTER: u32 = 0;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    init_common(&mut handover);
    kprintln!("Hello, World!");
    kprintln!("Entries: {}", handover.mmap_entries);
    for ent in handover.memory_map() {
        kcolorchange!(bg: Color::Black, fg: Color::White);
        kprint!("{:#?}", ent.ty);
        kcolorchange!(bg: Color::Black, fg: Color::Purple);
        kprint!(
            " {}kb = {}mb = {}gb\n",
            ent.page_count * 4096 / 1024,
            ent.page_count * 4096 / 1024 / 1024,
            ent.page_count * 4096 / 1024 / 1024 / 1024,
        );
    }

    panic!("Oh no... Something happened!");
    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // This is not performant code - This creates a pretty panic-screen.
    // This is the last thing that this OS does - It does not have to be performant
    use core::fmt::Write;

    unsafe {
        FRAMEBUFFER_GUARD
            .lock()
            .assume_init_mut()
            .set_color(0x0827F5_u32, 0xfac102_u32)
    }
    let (file, line, col) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        None => ("Unknown", 0, 0),
    };
    unsafe {
        let width = FRAMEBUFFER_GUARD.lock().assume_init_mut().resolution().0;
        let height = FRAMEBUFFER_GUARD.lock().assume_init_mut().resolution().1;
        FRAMEBUFFER_GUARD
            .lock()
            .assume_init_mut()
            .clear_color(0x0827F5_u32);

        let by_how_much = if width / 2 > 200 && height / 2 > 200 {
            200_usize
        } else {
            0_usize
        };

        FRAMEBUFFER_GUARD
            .lock()
            .assume_init_mut()
            .set_location(height / 2 - by_how_much, width / 2 - by_how_much);

        FRAMEBUFFER_GUARD
            .lock()
            .assume_init_mut()
            .set_column_starting_point(width / 2 - by_how_much);
    }

    kprintln!("*+~*+~*+~*+~*+~*+~*+~*+~*+~ Kernel Panic *+~*+~*+~*+~*+~*+~*+~*+~");
    kprintln!();
    kprintln!("At: ");
    kprintln!("\t-> File    :: {}", file);
    kprintln!("\t-> Line    :: {}", line);
    kprintln!("\t-> Column  :: {}", col);
    kprintln!("Message: ");
    if let Some(args) = info.message() {
        kprint!("\t-> ");
        unsafe {
            FRAMEBUFFER_GUARD
                .lock()
                .assume_init_mut()
                .write_fmt(*args)
                .unwrap()
        }
        kprint!("\n");
    } else {
        kprintln!("\t-> No Message provided");
    }
    kprintln!();
    kprintln!("*+~*+~*+~*+~*+~*+~*+~*+~*+~ Panic End *+~*+~*+~*+~*+~*+~*+~*+~*+~");
    loop {
        unsafe {
            asm!("cli");
        }
    }
}
