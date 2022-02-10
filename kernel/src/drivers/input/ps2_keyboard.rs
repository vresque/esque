use keyboard_layout::{translator, Modifier, RELEASED_COUNTERPART};
use spin::Mutex;

use crate::config;
use crate::framebuffer::FRAMEBUFFER_GUARD;
use crate::{
    interrupts::interrupt_frame::InterruptFrame,
    iobus::inb,
    kprintln,
    pic::{end_main_pic, PicPort},
};
use core::fmt::Write;

pub extern "x86-interrupt" fn ps2_keyboard_int_handler(_a: InterruptFrame) {
    // Get Keyboard Scancode
    let scancode = inb(PicPort::Ps2KeyboardScancodePort);
    handle_keyboard(scancode);
    end_main_pic();
}

static MODIFIER_STATE: Mutex<[bool; 2]> = Mutex::new([false, false]);
pub fn switch_state_of_mod(modi: u8) {
    let num = match modi {
        Modifier::LeftShift => 0,
        Modifier::RightShift => 1,
        _ => 0,
    };
    let prev = *MODIFIER_STATE.lock().get(num).unwrap();
    change_state_of_mod_to(modi, !prev);
}

pub fn change_state_of_mod_to(modi: u8, value: bool) {
    let num = match modi {
        Modifier::LeftShift => 0,
        Modifier::RightShift => 1,
        _ => 0,
    };
    *MODIFIER_STATE.lock().get_mut(num).unwrap() = value;
}

pub fn handle_keyboard(scancode: u8) {
    // Special Keys
    match scancode {
        Modifier::LeftShift => change_state_of_mod_to(Modifier::LeftShift, true),
        _ if scancode == Modifier::LeftShift + RELEASED_COUNTERPART => {
            change_state_of_mod_to(Modifier::LeftShift, false)
        }

        Modifier::RightShift => change_state_of_mod_to(Modifier::LeftShift, true),
        _ if scancode == Modifier::RightShift + RELEASED_COUNTERPART => {
            change_state_of_mod_to(Modifier::RightShift, false)
        }

        Modifier::Spacebar => unsafe {
            FRAMEBUFFER_GUARD
                .lock()
                .assume_init_mut()
                .write_char(' ')
                .unwrap();
        },

        Modifier::Enter => {
            kprintln!("\n");
        }
        Modifier::BackSpace => {
            unsafe {
                FRAMEBUFFER_GUARD.lock().assume_init_mut().clear_last_char();
            };
        }
        _ => {
            let is_lshift_pressed = MODIFIER_STATE.lock()[1];
            let uppercase = MODIFIER_STATE.lock()[0] | is_lshift_pressed;
            let ascii =
                translator::translate_from_u8(scancode, uppercase, config().layout as usize);
            // NULLs cannot be displayed
            if ascii != 0 as char {
                unsafe {
                    FRAMEBUFFER_GUARD
                        .lock()
                        .assume_init_mut()
                        .write_char(ascii)
                        .unwrap();
                }
            }
        }
    }
}
