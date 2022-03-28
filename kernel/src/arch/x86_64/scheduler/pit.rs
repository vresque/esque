use spin::mutex::Mutex;
use volatile::Volatile;

use crate::{
    arch::interrupts::interrupt_frame::InterruptFrame,
    arch::pic::{end_main_pic, PicPort},
    iobus::{io_wait, outb},
};

/// Without the Volatile, the compiler *may* optimize sleep into an infinite loop
pub static TIME_SINCE_BOOT: Mutex<Volatile<f64>> = Mutex::new(Volatile::new_rw(0.0));

pub const PIT_INTERRUPT: u64 = 0x20;

/// How often the PIT Chip oscillates per second
const BASE_FREQUENCY: u64 = 1193182;

pub const DIVISOR_MAX: u16 = 65535;
// No reason for this, but everything below this is *very* fast
const DIVISOR_MIN: u16 = 100;
/// The Divisor - The higher the value, the less the frequency, the more time between interrupts
static DIVISOR: Mutex<u16> = Mutex::new(DIVISOR_MAX);

/// # Sleep Seconds
/// Sleep for `seconds` seconds.
/// ## Parameters
/// - `seconds`: f64 = The amount of time to sleep in seconds
///
/// ## Notes
/// If milliseconds are desired, `msleep()` should be used, for microseconds `usleep` and if nanoseconds are desired `nanosleep()` should be used
pub fn sleep(seconds: f64) {
    let start = TIME_SINCE_BOOT.lock().read();
    while TIME_SINCE_BOOT.lock().read() < (start + seconds) {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// # Millisecond-Sleep
/// Sleep for `millis` milliseconds.
/// ## Parameters
/// - `millis`: u64 = The amount of time to sleep in milliseconds
///
/// ## Notes
/// If seconds are desired, `sleep()` should be used, for microseconds `usleep` and if nanoseconds are desired `nanosleep()` should be used
pub fn msleep(millis: u64) {
    sleep((millis / 1000) as f64)
}

/// # Microsecond-Sleep
/// Sleep for `micros` microseconds.
/// ## Parameters
/// - `micros`: u64 = The amount of time to sleep in microseconds
///
/// ## Notes
/// If seconds are desired, `sleep()` should be used, for milliseconds `msleep` and if nanoseconds are desired `nanosleep()` should be used
pub fn usleep(micros: u64) {
    msleep(micros / 1000)
}

/// # Nanosecond-Sleep
/// Sleep for `nanos` nanoseconds.
/// ## Parameters
/// - `nanoseconds`: u128 = The amount of time to sleep in nanoseconds
///
/// ## Notes
/// If seconds are desired, `sleep()` should be used, for milliseconds `msleep` and if microseconds are desired `usleep()` should be used
pub fn nanosleep(nanoseconds: u64) {
    usleep(nanoseconds / 1000)
}

/// # Set Divisor
/// Sets the divisor to a new value
/// ## Params
/// - `divisor`: u16 = The new divisor value
pub fn set_divisor(divisor: u16) {
    let divisor = if divisor < DIVISOR_MIN {
        DIVISOR_MIN
    } else {
        divisor
    };
    *DIVISOR.lock() = divisor;

    let divisor_lo = (divisor & 0x00ff) as u8;
    outb(PicPort::PitPort, divisor_lo);
    io_wait();

    let divisor_hi = ((divisor & 0xff00) >> 8) as u8;
    outb(PicPort::PitPort, divisor_hi);
    io_wait();
}

/// # Get Frequency
/// Gets the interrupts produced by the PIT per second
/// ## Returns
/// - u64 = The frequence
pub fn get_frequency() -> u64 {
    BASE_FREQUENCY / *DIVISOR.lock() as u64
}

/// # Set Frequency
/// Sets the interrupts produced by the PIT per second
/// ## Params
/// - `freq`: u64 = The new frequency
pub fn set_frequency(freq: u64) {
    set_divisor((BASE_FREQUENCY / freq) as u16);
}

/// # Tick
/// Is called everytime the PIT calls an interrupt (`get_frequency` times per second)
///
/// ## Notes
/// Do not call manually
pub fn tick() {
    let old_value = TIME_SINCE_BOOT.lock().read();
    TIME_SINCE_BOOT
        .lock()
        .write(old_value + (1f64 / get_frequency() as f64));
}

pub extern "x86-interrupt" fn pit_interrupt_handler(_a: InterruptFrame) {
    tick();
    end_main_pic();
}
