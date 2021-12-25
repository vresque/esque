#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct InterruptRegisters {
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rax: u64,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PreservedRegisters {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IRetRegisters {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct InterruptFrame {
    pub _ignore: u64,
    pub preserved: PreservedRegisters,
    pub registers: InterruptRegisters,
    pub iret: IRetRegisters,
}
