use alloc::{borrow::ToOwned, vec::Vec};

pub mod launchpad;
pub mod pid;
pub mod signal;

pub fn jump_to_userspace(location: u32, argv: Vec<&str>, stack: u32) {
    unsafe fn jump_to_userspace_inner(
        location: *const u32,
        argc: u32,
        argv: *const *const u8,
        stack: *const u32,
    ) {
        #[allow(named_asm_labels)]
        {
            core::arch::asm!(
                "
                sysret
            "
            );
        }
        //core::arch::asm!("wrmsr");
    }

    unsafe {
        let loc = location as *mut u32;
        let argc = argv.len() + 1; // The first one is the name itself
        let args = argv
            .into_iter()
            .map(|x| x.as_ptr())
            .collect::<Vec<*const u8>>()
            .as_ptr();
        let stack_ptr = stack as *mut u32;
        jump_to_userspace_inner(loc, argc as u32, args, stack_ptr);
    }
}
