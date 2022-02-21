#![no_std]
#![no_main]
// Features ---
#![feature(asm_const)]
#![feature(arbitrary_enum_discriminant)]
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
pub mod gdt;
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
pub mod interrupts;
pub mod iobus;
pub mod pic;
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
