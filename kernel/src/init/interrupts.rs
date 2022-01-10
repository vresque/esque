use bks::Handover;

use crate::drivers::input::ps2::ps2_keyboard_int_handler;
use crate::interrupts::exceptions::IDTException::InvalidTSS;
use crate::interrupts::exceptions::{ExceptionHandler, Exception};
use crate::interrupts::interrupt_frame::InterruptFrame;
use crate::interrupts::set_interrupt_handler;
use crate::memory::paging::page_frame_allocator::request_page;
use crate::pic::PicInterrupt;
use crate::scheduler::pit::{pit_interrupt_handler, PIT_INTERRUPT};
use crate::{info, success};
use crate::{interrupts::exceptions::IDTException, kprintln};
use core::arch::asm;

use crate::interrupts::idt::{upload_idt, IDTRegister, IDT_REGISTER};

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

extern "x86-interrupt" fn generic_fault_handler(_frame: InterruptFrame) {
    panic!("A fault occured");
}

extern "x86-interrupt" fn page_fault_handler(_frame: InterruptFrame) {
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

extern "x86-interrupt" fn double_fault_handler(_a: InterruptFrame) {
    panic!("Double Fault detected");
}

extern "x86-interrupt" fn general_protection_fault_handler(_a: InterruptFrame) {
    panic!("General Protection Fault detected");
}

pub fn init_interrupts(_: &mut Handover) {
    info!("Initializing Interrupts");
    let idtr_limit = 0x0FFF;
    let idtr_offset = request_page::<u64>();
    let idtr = IDTRegister::new(idtr_limit, *idtr_offset);
    unsafe {
        IDT_REGISTER.force_unlock();
    }
    IDT_REGISTER.lock().write(idtr);
    crate::debug!("{:#?}", IDT_REGISTER.lock().assume_init_mut());

    crate::scheduler::pit::set_divisor(65535 / 3);

    set_interrupt_handler(IDTException::PageFault as u64, page_fault_handler);
    set_interrupt_handler(IDTException::DoubleFault as u64, double_fault_handler);
    set_interrupt_handler(
        IDTException::GeneralProtectionFault as u64,
        general_protection_fault_handler,
    );

    set_interrupt_handler(IDTException::DivideByZero as u64, generic_fault_handler);
    set_interrupt_handler(IDTException::Debug as u64, generic_fault_handler);
    set_interrupt_handler(IDTException::NonMaskable as u64, generic_fault_handler);
    set_interrupt_handler(IDTException::Breakpoint as u64, generic_fault_handler);
    set_interrupt_handler(IDTException::Overflow as u64, generic_fault_handler);
    set_interrupt_handler(
        IDTException::BoundRangeExceeded as u64,
        generic_fault_handler,
    );
    set_interrupt_handler(IDTException::InvalidOpcode as u64, generic_fault_handler);
    set_interrupt_handler(
        IDTException::DeviceNotAvailable as u64,
        generic_fault_handler,
    );
    set_interrupt_handler(IDTException::InvalidTSS as u64, ExceptionHandler::<InvalidTSS>::handle() as u64);
    set_interrupt_handler(
        IDTException::SegmentNotPresent as u64,
        generic_fault_handler,
    );
    set_interrupt_handler(
        IDTException::StackSegmentFault as u64,
        generic_fault_handler,
    );
    set_interrupt_handler(
        IDTException::X87FloatingPointException as u64,
        generic_fault_handler,
    );
    set_interrupt_handler(IDTException::AlignmentCheck as u64, generic_fault_handler);
    set_interrupt_handler(IDTException::MachineCheck as u64, generic_fault_handler);
    set_interrupt_handler(
        IDTException::SIMDFloatingPointException as u64,
        generic_fault_handler,
    );
    set_interrupt_handler(
        IDTException::VirtualizationException as u64,
        generic_fault_handler,
    );
    set_interrupt_handler(
        IDTException::ControlProtection as u64,
        generic_fault_handler,
    );
    set_interrupt_handler(
        IDTException::HypervisorInjection as u64,
        generic_fault_handler,
    );
    set_interrupt_handler(
        IDTException::VMMCommunicationException as u64,
        generic_fault_handler,
    );
    set_interrupt_handler(
        IDTException::SecurityException as u64,
        generic_fault_handler,
    );

    // Add PS2 Interrupt Handler
    set_interrupt_handler(
        PicInterrupt::KeyboardInterrupt as u64,
        ps2_keyboard_int_handler,
    );

    // Set PIT Interrupt Handler
    set_interrupt_handler(PIT_INTERRUPT as u64, pit_interrupt_handler);

    // Loading the IDT
    info!("Loading IDTR");
    unsafe {
        // Load the IDT
        upload_idt(IDT_REGISTER.lock().assume_init_mut());
    }

    success!("Finished preparing interrupts");
}
