use bks::Handover;

use crate::syscall::syscall_handler;
use crate::{
    gdt::GdtEntryType,
    iobus::msr::{read_msr, write_msr, MsrRegister},
};

pub fn init_syscalls(_: &mut Handover) {
    let syscall_base = GdtEntryType::KernelCode << 3;
    let sysret_base = (GdtEntryType::UserCode32Unused << 3) | 3;

    let star_hi = syscall_base as u32 | ((sysret_base as u32) << 16);

    write_msr(MsrRegister::Star, (star_hi as u64) << 32);
    write_msr(MsrRegister::LStar, syscall_handler as u64);

    // Clear Trap Flag
    write_msr(MsrRegister::SyscallMask, 0x300);

    let efer_val = read_msr(MsrRegister::Efer);
    write_msr(MsrRegister::Efer, efer_val | 1);
}
