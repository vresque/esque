#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
static _OFFSET: u64 = 0;

#[no_mangle]
static _APP_START: u64 = 0;

#[no_mangle]
static _APP_END: u64 = 0;

static mut ARGC: u64 = 0;

// static ARGC: Mutex<u64> = Mutex::new(0);
// static ARGV: Mutex<u64> = Mutex::new(0);

extern "Rust" {
    fn main() -> u64;
}
#[no_mangle]
pub extern "C" fn _start(argc: u64, argv: u64, msg_base: u64) -> u64 {
    unsafe {
        ARGC = argc;
    }
    //    *ARGC.lock() = argc;
    //    *ARGV.lock() = argv;
    unsafe { main() }
}

pub fn argc() -> u64 {
    return unsafe { ARGC };
}

#[panic_handler]
pub fn _panic_handler(_: &PanicInfo) -> ! {
    loop {}
}
