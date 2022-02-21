use crate::iobus::msr::{read_msr, MsrRegister};

#[repr(packed)]
pub struct Registers {
    // PRESERVED REGISTERS ----------------
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
    // SCRATCH REGISTERS -----------------
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rax: u64,
    // IRET REGISTERS --------------------
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl core::fmt::Display for Registers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "rflag: {:016x}", self.rflags);
        writeln!(f, "cs:    {:016x}", self.cs);
        writeln!(f, "rip:   {:016x}", self.rip);

        if self.cs & 0b11 != 0b00 {
            writeln!(f, "rsp:   {:016x}", self.rsp)?;
            writeln!(f, "ss:    {:016x}", self.ss)?;
        }

        let fsbase = read_msr(MsrRegister::FsBase);
        let gsbase = read_msr(MsrRegister::KernelBase);
        let kgsbase = read_msr(MsrRegister::GsBase);
        writeln!(f, "fsbase: {:016x}", fsbase);
        writeln!(f, "gsbase: {:016x}", gsbase);
        writeln!(f, "kgsbase: {:016x}", kgsbase);
        writeln!(f, "rax:   {:016x}", self.rax)?;
        writeln!(f, "rcx:   {:016x}", self.rcx)?;
        writeln!(f, "rdx:   {:016x}", self.rdx)?;
        writeln!(f, "rdi:   {:016x}", self.rdi)?;
        writeln!(f, "rsi:   {:016x}", self.rsi)?;
        writeln!(f, "r8:    {:016x}", self.r8)?;
        writeln!(f, "r9:    {:016x}", self.r9)?;
        writeln!(f, "r10:   {:016x}", self.r10)?;
        writeln!(f, "r11:   {:016x}", self.r11)?;
        writeln!(f, "rbx:   {:016x}", self.rbx)?;
        writeln!(f, "rbp:   {:016x}", self.rbp)?;
        writeln!(f, "r12:   {:016x}", self.r12)?;
        writeln!(f, "r13:   {:016x}", self.r13)?;
        writeln!(f, "r14:   {:016x}", self.r14)?;
        writeln!(f, "r15:   {:016x}", self.r15)?;
        Ok(())
    }
}
