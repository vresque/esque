use super::interrupt_frame::InterruptFrame;

pub enum ExceptionType {
    Fault,
    Abort,
    Interrupt,
    Trap,
}

enum_with_options::const_enum! {
    pub enum IDTException: u8 => {
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

    impl {
        pub fn error_code(self: &Me) {
            match *self {
                IDTException::DivideByZero => "#DE",
                IDTException::Debug => "#DB",
                IDTException::NonMaskable => "-",
                IDTException::Breakpoint => "#BP",
                IDTException::Overflow => "#OF",
                IDTException::BoundRangeExceeded => "#BR",
                IDTException::InvalidOpcode => "#UD",
                IDTException::DeviceNotAvailable => "#NM",
                IDTException::DoubleFault => "#DF",
                IDTException::InvalidTSS => "#TS",
                IDTException::SegmentNotPresent => "#NP",
                IDTException::StackSegmentFault => "#SS",
                IDTException::GeneralProtectionFault => "#GP",
                IDTException::PageFault => "#PF",
                IDTException::X87FloatingPointException => "MF",
                IDTException::AlignmentCheck => "#AC",
                IDTException::MachineCheck => "#MC",
                IDTException::SIMDFloatingPointException => "#XM",
                IDTException::VirtualizationException => "#VE",
                IDTException::ControlProtection => "#CP",
                IDTException::HypervisorInjection => "#HV",
                IDTException::VMMCommunicationException => "#VC",
                IDTException::SecurityException => "#SX",
                _ => "Unknown",
            }
        }

        pub fn type_(self: &Me) -> ExceptionType {
            match *self {
                IDTException::DivideByZero => todo!(),
                IDTException::Debug => todo!(),
                IDTException::NonMaskable => todo!(),
                IDTException::Breakpoint => todo!(),
                IDTException::Overflow => todo!(),
                IDTException::BoundRangeExceeded => todo!(),
                IDTException::InvalidOpcode => todo!(),
                IDTException::DeviceNotAvailable => todo!(),
                IDTException::DoubleFault => todo!(),
                IDTException::InvalidTSS => todo!(),
                IDTException::SegmentNotPresent => todo!(),
                IDTException::StackSegmentFault => todo!(),
                IDTException::GeneralProtectionFault => todo!(),
                IDTException::PageFault => todo!(),
                IDTException::X87FloatingPointException => todo!(),
                IDTException::AlignmentCheck => todo!(),
                IDTException::MachineCheck => todo!(),
                IDTException::SIMDFloatingPointException => todo!(),
                IDTException::VirtualizationException => todo!(),
                IDTException::ControlProtection => todo!(),
                IDTException::HypervisorInjection => todo!(),
                IDTException::VMMCommunicationException => todo!(),
                IDTException::SecurityException => todo!(),
                _ => todo!(),
            }
        }
    }
}

pub struct ExceptionHandler<const T: u8>;

pub fn exception_panic(n: u8) -> ! {
    panic!("Lol!");
    loop {}
}

impl ExceptionHandler<{ IDTException::PageFault }> {
    pub fn handle() {
        exception_panic(IDTException::PageFault)
    }
}
