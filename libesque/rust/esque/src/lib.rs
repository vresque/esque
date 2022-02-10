#![no_std]


extern {
    fn main();
}

#[no_mangle]
pub fn _start() -> i32 {
    unsafe { main(); };
    return 22;
}

#[panic_handler]
pub fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}