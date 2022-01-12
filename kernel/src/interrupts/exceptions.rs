pub use self::IDTException::*;

use super::interrupt_frame::InterruptFrame;
use core::arch::asm;

#[allow(unused)]
pub enum ExceptionType {
    Fault,
    Abort,
    Interrupt,
    Trap,
}

enumtastic::const_enum! {
    pub enum IDTException: usize => {
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
        pub fn error_code(me: &Me) -> &str {
             match *me {
                 DivideByZero => "#DE",
                 Debug => "#DB",
                 NonMaskable => "-",
                 Breakpoint => "#BP",
                 Overflow => "#OF",
                 BoundRangeExceeded => "#BR",
                 InvalidOpcode => "#UD",
                 DeviceNotAvailable => "#NM",
                 DoubleFault => "#DF",
                 InvalidTSS => "#TS",
                 SegmentNotPresent => "#NP",
                 StackSegmentFault => "#SS",
                 GeneralProtectionFault => "#GP",
                 PageFault => "#PF",
                 X87FloatingPointException => "MF",
                 AlignmentCheck => "#AC",
                 MachineCheck => "#MC",
                 SIMDFloatingPointException => "#XM",
                 VirtualizationException => "#VE",
                 ControlProtection => "#CP",
                 HypervisorInjection => "#HV",
                 VMMCommunicationException => "#VC",
                 SecurityException => "#SX",
                 _ => "Unknown",
             }
        }

        pub fn type_(me: &Me) -> super::ExceptionType {
             match *me {
                 DivideByZero => todo!(),
                 Debug => todo!(),
                 NonMaskable => todo!(),
                 Breakpoint => todo!(),
                 Overflow => todo!(),
                 BoundRangeExceeded => todo!(),
                 InvalidOpcode => todo!(),
                 DeviceNotAvailable => todo!(),
                 DoubleFault => todo!(),
                 InvalidTSS => todo!(),
                 SegmentNotPresent => todo!(),
                 StackSegmentFault => todo!(),
                 GeneralProtectionFault => todo!(),
                 PageFault => todo!(),
                 X87FloatingPointException => todo!(),
                 AlignmentCheck => todo!(),
                 MachineCheck => todo!(),
                 SIMDFloatingPointException => todo!(),
                 VirtualizationException => todo!(),
                 ControlProtection => todo!(),
                 HypervisorInjection => todo!(),
                 VMMCommunicationException => todo!(),
                 SecurityException => todo!(),
                 _ => todo!(),
             }
        }
    }
}

pub trait Exception<const T: usize> {
    extern "x86-interrupt" fn handle(frame: InterruptFrame);
    fn get_name() -> &'static str {
        stringify!(T)
    }

    fn get_error_code() -> &'static str {
        IDTException::error_code(&T)
    }
}
pub struct ExceptionHandler<const T: usize>;

macro_rules! impl_generic_exception_handler {
    (
        $(
            $op:ident,
        )*
    ) => {
        $(
            impl Exception<$op> for ExceptionHandler<$op> {
                extern "x86-interrupt" fn handle(frame: InterruptFrame) {
                    panic!("Triggered Fault {} ({:#x?}) with opcode {}", stringify!($op), $op, IDTException::error_code(&$op))
                }
            }
        )*
    }
}

impl_generic_exception_handler! {
    DivideByZero,
    Debug,
    NonMaskable,
    Breakpoint,
    Overflow,
    BoundRangeExceeded,
    InvalidOpcode,
    DeviceNotAvailable,
    InvalidTSS,
    SegmentNotPresent,
    StackSegmentFault,
    // 0xF = Reserved,
    X87FloatingPointException ,
    AlignmentCheck ,
    MachineCheck ,
    DoubleFault,
    SIMDFloatingPointException ,
    VirtualizationException ,
    ControlProtection ,
    HypervisorInjection ,
    VMMCommunicationException ,
    SecurityException ,
    // 0x1F = Reserved,
    // TripleFault does not have a code,
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct PageFaultErrorCode: u64 {
        const PAGE_PROTECTON_VIOLATION = 1;
        const CAUSED_BY_WRITE_ACCESS = 1 << 1;
        const USER_MODE = 1 << 2;
        const MALFORMED_TABLE_RESERVED_WRITE = 1 << 3;
        const INSTRUCTION_FETCH = 1 << 4;
    }
}

impl Exception<PageFault> for ExceptionHandler<PageFault> {
    extern "x86-interrupt" fn handle(frame: InterruptFrame) {
        let code: u64;
        let cr2: u64;
        unsafe {
            asm!("mov {}, rsp", out(reg) code);
            asm!("mov {}, cr2", out(reg) cr2);
        };
        let err = PageFaultErrorCode::from_bits_truncate(code);
        panic!(
            "Page Fault Occured at address {:#x?} with code {:#?}",
            cr2, err
        );
    }
}

impl Exception<GeneralProtectionFault> for ExceptionHandler<GeneralProtectionFault> {
    extern "x86-interrupt" fn handle(frame: InterruptFrame) {}
}
