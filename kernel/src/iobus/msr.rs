enumtastic::const_enum! {
    // https://wiki.osdev.org/PIC#Programming_the_PIC_chips
    pub enum MsrRegister: u64 => {
        Apic = 0x1B,
        Efer = 0xC0000080,
        Star = 0xC0000081,
        LStar = 0xC0000082,
        CompatStar = 0xC0000083,
        SyscallMask = 0xC0000084,
        FsBase = 0xC0000100,
        GsBase = 0xC0000101,
        KernelBase = 0xc0000102,
    }

    impl {}
}

pub fn read_msr(register: u64) -> u64 {
    let lo: u32;
    let hi: u32;

    unsafe {
        core::arch::asm!("rdmsr", out("eax") lo, out("edx") hi, in("ecx")register, options(nomem));
    }
    ((hi as u64) << 32) | (lo as u64)
}

pub fn write_msr(register: u64, value: u64) {
    let lo: u32 = value as u32;
    let hi = (value >> 32) as u32;
    unsafe {
        core::arch::asm!("wrmsr", in("ecx") register, in("eax") lo, in ("edx") hi, options(nomem))
    };
}
