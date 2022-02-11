use crate::framebuffer::clear_screen;
use crate::framebuffer::FRAMEBUFFER_GUARD;
use crate::kcolorchange;
use crate::{kprint, kprintln};
use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // This is not performant code - This creates a pretty panic-screen.
    // This is the last thing that this OS does - It does not have to be performant
    use core::fmt::Write;

    kcolorchange!(bg: 0x020936_u32, fg: 0xfac102_u32);

    let (file, line, col) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        None => ("Unknown", 0, 0),
    };
    unsafe {
        let width = FRAMEBUFFER_GUARD.lock().assume_init_mut().resolution().0;
        let height = FRAMEBUFFER_GUARD.lock().assume_init_mut().resolution().1;
        clear_screen(0x020936_u32);

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
    unsafe {
        comasm::clear_interrupts();
    };
    loop {
        unsafe {
            comasm::halt();
        }
    }
}
