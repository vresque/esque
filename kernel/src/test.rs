use crate::success;

#[cfg(feature = "tests")]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

#[cfg(feature = "tests")]
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

pub struct RustTest {
    pub func: fn(),
    pub name: &'static str,
}

#[esqtest::esqtest]
pub fn test() {
    success!("printing works!");
}

#[cfg(feature = "tests")]
pub fn test_runner(tests: &[&RustTest]) {
    let mut passed_tests: u32 = 0;
    use crate::framebuffer::clear_screen;
    use crate::{info, success};

    clear_screen(0x0_u32);
    info!("Running {} tests...", tests.len());
    for test in tests {
        (test.func)();
        success!("{}.............. ok", test.name);
        passed_tests += 1;
    }

    success!("Passed {} tests", passed_tests);

    QemuExitCode::Success.exit_qemu();
}

#[cfg(not(feature = "tests"))]
pub fn test_runner(tests: &[&dyn Fn()]) {}
