pub enum ExceptionType {
    Fault,
    Abort,
    Interrupt,
    Trap
}

#[repr(u8)]
pub enum IDTExceptions {
    DivideByZero = 0x0,
    Debug = 0x1,
    NonMaskable = 0x2,
    Breakpoint = 0x3,
    Overflow = 0x4,
    BoundRangeExceeded = 0x5,
    InvalidOpcode = 0x6,
    DeviceNotAvailable = 0x7,
    DoubleFault = 0x8,
    InvalidTSS = 0xA,
    SegmentNotPresent = 0xB,
    StackSegmentFault = 0xC,
    GeneralProtectionFault = 0xD,
    PageFault = 0xE,
    // 0xF = RESERVED
    X87FloatingPointException = 0x10,
    AlignmentCheck = 0x11,
    MachineCheck = 0x12,
    SIMDFloatingPointException = 0x13,
    VirtualizationException = 0x14,
    ControlProtection = 0x15,
    // 0x16 - 0x1B = RESERVED
    HypervisorInjection = 0x1C,
    VMMCommunicationException = 0x1D,
    SecurityException = 0x1E,
    // 0x1F = RESERVED
    // TripleFault does not have a code
}

impl Into<u8> for IDTExceptions {
    fn into(self) -> u8 {
        self as u8
    }
}

impl IDTExceptions {
    fn error_code(&self) -> &str {
        match *self {
            IDTExceptions::DivideByZero => "#DE",
            IDTExceptions::Debug => "#DB",
            IDTExceptions::NonMaskable => "-",
            IDTExceptions::Breakpoint => "#BP",
            IDTExceptions::Overflow => "#OF",
            IDTExceptions::BoundRangeExceeded => "#BR",
            IDTExceptions::InvalidOpcode => "#UD",
            IDTExceptions::DeviceNotAvailable => "#NM",
            IDTExceptions::DoubleFault => "#DF",
            IDTExceptions::InvalidTSS => "#TS",
            IDTExceptions::SegmentNotPresent => "#NP",
            IDTExceptions::StackSegmentFault => "#SS",
            IDTExceptions::GeneralProtectionFault => "#GP",
            IDTExceptions::PageFault => "#PF",
            IDTExceptions::X87FloatingPointException => "MF",
            IDTExceptions::AlignmentCheck => "#AC",
            IDTExceptions::MachineCheck => "#MC",
            IDTExceptions::SIMDFloatingPointException => "#XM",
            IDTExceptions::VirtualizationException => "#VE",
            IDTExceptions::ControlProtection => "#CP",
            IDTExceptions::HypervisorInjection => "#HV",
            IDTExceptions::VMMCommunicationException => "#VC",
            IDTExceptions::SecurityException => "#SX",
        }
    }

    pub fn type_(&self) -> ExceptionType {
        match *self {
            IDTExceptions::DivideByZero => todo!(),
            IDTExceptions::Debug => todo!(),
            IDTExceptions::NonMaskable => todo!(),
            IDTExceptions::Breakpoint => todo!(),
            IDTExceptions::Overflow => todo!(),
            IDTExceptions::BoundRangeExceeded => todo!(),
            IDTExceptions::InvalidOpcode => todo!(),
            IDTExceptions::DeviceNotAvailable => todo!(),
            IDTExceptions::DoubleFault => todo!(),
            IDTExceptions::InvalidTSS => todo!(),
            IDTExceptions::SegmentNotPresent => todo!(),
            IDTExceptions::StackSegmentFault => todo!(),
            IDTExceptions::GeneralProtectionFault => todo!(),
            IDTExceptions::PageFault => todo!(),
            IDTExceptions::X87FloatingPointException => todo!(),
            IDTExceptions::AlignmentCheck => todo!(),
            IDTExceptions::MachineCheck => todo!(),
            IDTExceptions::SIMDFloatingPointException => todo!(),
            IDTExceptions::VirtualizationException => todo!(),
            IDTExceptions::ControlProtection => todo!(),
            IDTExceptions::HypervisorInjection => todo!(),
            IDTExceptions::VMMCommunicationException => todo!(),
            IDTExceptions::SecurityException => todo!(),
        }
    }
}