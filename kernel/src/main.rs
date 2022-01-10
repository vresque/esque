#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(rustc_private)]
#![feature(abi_x86_interrupt)]
#![allow(unstable_features)]
#![feature(alloc_error_handler)]
#![allow(unused_unsafe)]
#![feature(int_log)]
#![allow(dead_code)]
// Functions and structs may not be used immediately, but may be added in case it will ever be needed
#![deny(unreachable_patterns)] // May lead to certain code not being reached due to bad code

extern crate alloc;

pub mod framebuffer;
pub mod gdt;
pub mod init;
pub mod memory;
pub mod panic;
pub use bks::Handover;
pub mod config;
pub mod drivers;
pub mod heap;
pub mod initramfs;
pub mod interrupts;
pub mod iobus;
pub mod pic;
pub mod scheduler;
pub mod userspace;
pub use config::config;
pub use esys::{
    ipc::{message::ptr::MessagePointer2, MessageContent},
    process::Process,
};
pub use userspace::pid::{KernelPid, Pid};
pub mod ipc;
pub mod syscall;

use crate::heap::{free, malloc, malloc_ptr};

pub const HEAP_ADDRESS: u64 = 0x0000900000;
pub const HEAP_LENGTH: usize = 0x1000;

#[no_mangle]
extern "sysv64" fn kmain(mut handover: Handover) -> u32 {
    init::config::init_config(&mut handover);
    init::gdt::init_gdt(&mut handover);
    // -#---#@@- Enables Print Macros -@@#---#-
    init::common::init_common(&mut handover);
    init::memory::init_paging(&mut handover);
    init::interrupts::init_interrupts(&mut handover);
    init::pic::init_pic(&mut handover);
    init::pit::init_pit(&mut handover);
    init::memory::map_memory(&mut handover);
    // -#---#@@- Enables Memory Allocation -@@#---#-
    init::memory::init_heap(&mut handover);
    drivers::init_fallback_drivers(&mut handover);
    initramfs::load_initramfs(&mut handover);

    // -#---#@@- Enables HAL System Calls -@@#---#-
    init::syscall::init_syscalls(&mut handover);
    initramfs::load_system_space_applications(&mut handover);

    unsafe { core::arch::asm!("mov rax, 23");
    core::arch::asm!("syscall");
}
    loop {}
}
