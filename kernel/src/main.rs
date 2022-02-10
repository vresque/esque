#![no_std]
#![no_main]
#![feature(arbitrary_enum_discriminant)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(rustc_private)]
#![feature(abi_x86_interrupt)]
#![allow(unstable_features)]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_unsafe)]
#![feature(int_log)]
#![feature(slice_pattern)]
#![allow(dead_code)]
// Functions and structs may not be used immediately, but may be added in case it will ever be needed
#![deny(unreachable_patterns)] // May lead to certain code not being reached due to bad code

extern crate alloc;
pub mod framebuffer;
pub mod gdt;
pub mod init;
pub mod memory;
pub mod panic;
pub mod pci;
pub use bks::Handover;
pub mod acpi;
pub mod config;
pub mod drivers;
pub mod heap;
pub mod initramfs;
pub mod interrupts;
pub mod iobus;
pub mod pic;
pub mod scheduler;
pub mod smp;
pub mod test;
pub mod userspace;
use bks::PAGE_SIZE;
pub use config::config;
pub use esys::process::Process;
pub use smp::Thread;
use userspace::launchpad::Launchpad;
pub use userspace::pid::{KernelPid, Pid};

pub mod ipc;
pub mod syscall;

pub const HEAP_ADDRESS: u64 = 0x0000900000;
pub const HEAP_LENGTH: usize = PAGE_SIZE as usize;

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
    init::acpi::init_acpi(&mut handover);
    // -#---#@@- Enables Memory Allocation -@@#---#-
    init::memory::init_heap(&mut handover);
    init::smp::init_smp(&mut handover);

    Thread::new(ipc::kernel_ipc_handler).launch();

    drivers::init_drivers(&mut handover);
    initramfs::load_initramfs(&mut handover);

    // -#---#@@- Enables System Calls -@@#---#-
    init::syscall::init_syscalls(&mut handover);
    initramfs::load_system_space_applications(&mut handover);

    Launchpad::new(&initramfs::fs::read_until_end("esqrc").unwrap(), true)
        .with_pid(Pid::force_new(1))
        .launch();

    #[cfg(test)]
    test_main();

    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
