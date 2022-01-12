use crate::success;

core::arch::global_asm!(include_str!("syscall.s"));

extern "C" {
    pub fn syscall_handler();
}

#[no_mangle]
pub extern "C" fn syscall_dispatcher() {}
