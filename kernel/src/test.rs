use esqtest::{all_good, check_neq};
use esys::ipc::IPCQueueHeader;

use crate::{address_of, emergency, heap::malloc, success};

#[repr(u32)]
#[cfg(feature = "harsh-tests")]
// QEMU will execute `exit(((code << 1) | 1))`
pub enum QemuExitCode {
    // Note: exit(0) is not supported
    // These values hold no significance at all, they were chosen at random
    // (The meaning of life)
    Success = 0x42,      // All Tests were good
    TotalFailure = 0x43, // At least 50% of the tests failed
    Mixed = 0x44,        // At least one test failed
}

#[cfg(feature = "harsh-tests")]
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
pub fn check_testing_framework() {
    check_neq!(0, 0);
    all_good!();
}

#[esqtest::test]
pub fn check_allocation() {
    check_neq!(address_of!(malloc::<IPCQueueHeader>()), 0);
    all_good!()
}

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
            emergency!("{}.............. failed", test.name);
            failed_tests += 1;
        }
    }

    info!("Testing has finished");
    success!("> Passed {} tests.", passed_tests);
    error!("> Failed {} tests.", failed_tests);
    info!("> Total Tests: {}", tests.len());
    let f_percentage = (failed_tests as f32 / tests.len() as f32) * 100.0;
    let p_percentage = (passed_tests as f32 / tests.len() as f32) * 100.0;

    #[cfg(feature = "harsh-tests")]
    // Must be here as it will otherwise be exclusive to the brackets below
    let exit_code: QemuExitCode = if f_percentage > 0.0 {
        // Bigger than 50?
        if f_percentage > 50.0 {
            QemuExitCode::TotalFailure
        } else {
            QemuExitCode::Mixed
        }
    } else {
        QemuExitCode::Success
    };

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

    #[cfg(feature = "harsh-tests")]
    exit_code.exit_qemu();
}
