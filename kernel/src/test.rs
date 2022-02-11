use esqtest::{all_good, check_neq};
use esys::ipc::IPCQueueHeader;

use crate::{address_of, heap::malloc, success};

#[cfg(feature = "test")]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

#[cfg(feature = "test")]
impl QemuExitCode {
    pub fn exit_qemu(self) -> ! {
        use crate::iobus::outl;

        outl(0xf4, self as u32);

        loop {
            unsafe {
                core::arch::asm!("hlt");
            }
        }
    }
}

#[esqtest::test]
pub fn check_printing() {
    success!("printing works!");
    all_good!()
}

#[esqtest::test]
pub fn check_allocation() {
    check_neq!(address_of!(malloc::<IPCQueueHeader>()), 0);
    all_good!()
}

#[cfg(feature = "test")]
pub fn test_runner(tests: &[&esqtest::RustTest]) {
    let mut passed_tests: u32 = 0;
    let mut failed_tests: u32 = 0;
    use crate::framebuffer::clear_screen;
    use crate::{error, info};

    clear_screen(0x0_u32);
    info!("Running {} tests...", tests.len());
    for test in tests {
        if (test.func)() == 0 {
            success!("{}.............. ok", test.name);
            passed_tests += 1;
        } else {
            error!("{}.............. failed", test.name);
            failed_tests += 1;
        }
    }

    info!("Testing has finished");
    success!("> Passed {} tests.", passed_tests);
    error!("> Failed {} tests.", failed_tests);
    info!("> Total Tests: {}", tests.len());
    let f_percentage = failed_tests / tests.len() as u32;
    let p_percentage = passed_tests / tests.len() as u32;

    if f_percentage < p_percentage {
        success!(
            "Passed {}% of the tests, failed {}%.",
            p_percentage,
            f_percentage
        );
    } else {
        error!(
            "Failed {}% of the tests, passed {}%.",
            f_percentage, p_percentage
        );
    }

    QemuExitCode::Success.exit_qemu();
}

#[cfg(not(feature = "test"))]
pub fn test_runner(_tests: &[&esqtest::RustTest]) {}
