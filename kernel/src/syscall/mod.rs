use crate::interrupts::register::Registers;

pub fn syscall(
    rax: u64,
    rdi: u64,
    rsi: u64,
    rdx: u64,
    r10: u64,
    r8: u64,
    r9: u64,
    rbp: u64,
    regs: &mut Registers,
) -> u64 {
    10
}
