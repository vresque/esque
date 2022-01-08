enumtastic::const_enum! {
    // https://wiki.osdev.org/PIC#Programming_the_PIC_chips
    pub enum MsrRegister: u64 => {
        MsrApic = 0x1B,
        MsrEfer = 0xC0000080,
        MsrStar = 0xC0000081,
        MsrLStar = 0xC0000082,
        MsrCompatStar = 0xC0000083,
        MsrSyscallMask = 0xC0000084,
        MsrFsBase = 0xC0000100,
        MsrGsBase = 0xC0000101,
        MsrKernelBase = 0xc0000102,    }

    impl {}
}

pub fn read_msr(_register: u64) -> u64 {
    2
    //let lo: u32;
    //let hi: u32;
    ////core::arch::asm!("rdmsr {}", in("c")register, out(reg_ad)lo, out("d")hi)
    //let fin = ((hi as u64) << 32u64 ) | lo as u64;
    //fin
}
