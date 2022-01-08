core::arch::global_asm!(include_str!("syscall.s"));
extern "C" {
    fn __syscall();
}

//extern "C" fn handle_syscall(reg: Registers, )

//pub struct Registers {}
