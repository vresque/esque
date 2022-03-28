#![no_std]
#![no_main]
// Features ---
#![feature(asm_const)]
#![feature(arbitrary_enum_discriminant)]
#![feature(thread_local)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(ptr_internals)]
#![feature(rustc_private)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]
#![feature(custom_test_frameworks)]
#![feature(int_log)]
#![feature(slice_pattern)]
#![feature(const_btree_new)]
// Allow ---
#![allow(unused_unsafe)]
#![allow(dead_code)]
#![allow(unstable_features)]
#![feature(naked_functions)]
// Deny ---
// Functions and structs may not be used immediately, but may be added in case it will ever be needed
#![deny(unreachable_patterns)] // May lead to certain code not being reached due to bad code

// Testing ----
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod arch;
pub mod error;
pub mod math;
extern crate alloc;

pub mod common;
pub mod framebuffer;
pub mod init;
pub mod memory;
pub mod panic;
pub mod pci;
pub mod tests;
use alloc::vec::Vec;
pub use bks::Handover;
pub mod acpi;
pub mod config;
pub mod device;
pub mod drivers;
pub mod env;
pub mod heap;
pub mod initramfs;
pub mod iobus;
pub mod scheduler;
pub mod smp;
#[cfg(test)]
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

pub fn main() -> ! {
    init::acpi::init_acpi();
    // -#---#@@- Enables Memory Allocation -@@#---#-
    init::heap::init_heap();

    Thread::new(ipc::kernel_ipc_handler).launch();

    drivers::init_drivers();
    initramfs::load_initramfs();

    // -#---#@@- Enables System Calls -@@#---#-
    arch::init::syscall::init_syscalls();
    initramfs::load_system_space_applications();

    for i in unsafe { initramfs::INITRAMFS.lock().assume_init_mut().entries() } {
        debug!("{:?}", i.filename);
    }

    #[cfg(test)]
    {
        success!("Running Tests...");
        test_main();
        success!("Ran all tests!");
    }

    Launchpad::new(
        &initramfs::fs::read_until_end("initramfs/esqrc").unwrap(),
        true,
    )
    .with_pid(Pid::force_new(1))
    .launch();

    let stack: Vec<u8> = Vec::with_capacity(0x1000);

    unsafe {
        load_userspace();
    }

    loop {
        unsafe { comasm::halt() };
    }
}

// This is the stub that will be called if no init is found
pub unsafe extern "C" fn kernel_userspace_stub(argc: u32, argv: *const *const u8) {
    core::arch::asm!(
        "
    nop
    nop
    nop"
    )
}

pub unsafe fn load_userspace() {}
