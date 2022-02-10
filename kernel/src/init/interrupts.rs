use bks::Handover;

use crate::drivers::input::ps2_keyboard::ps2_keyboard_int_handler;
use crate::drivers::input::ps2_mouse::ps2_mouse_interrupt_handler;
use crate::interrupts::exceptions::IDTException::*;
use crate::interrupts::exceptions::{Exception, ExceptionHandler};
use crate::interrupts::set_interrupt_handler;
use crate::memory::paging::page_frame_allocator::request_page;
use crate::pic::PicInterrupt;
use crate::scheduler::pit::{pit_interrupt_handler, PIT_INTERRUPT};
use crate::{info, success};
use crate::{interrupts::exceptions::IDTException, kprintln};

use crate::interrupts::idt::{upload_idt, IDTRegister, IDT_REGISTER};

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

    set_interrupt_handler(
        IDTException::PageFault as u64,
        ExceptionHandler::<PageFault>::handle,
    );
    set_interrupt_handler(
        IDTException::DoubleFault as u64,
        ExceptionHandler::<DoubleFault>::handle,
    );
    set_interrupt_handler(
        IDTException::GeneralProtectionFault as u64,
        ExceptionHandler::<GeneralProtectionFault>::handle,
    );

    set_interrupt_handler(
        IDTException::DivideByZero as u64,
        ExceptionHandler::<DivideByZero>::handle,
    );
    set_interrupt_handler(
        IDTException::Debug as u64,
        ExceptionHandler::<Debug>::handle,
    );
    set_interrupt_handler(
        IDTException::NonMaskable as u64,
        ExceptionHandler::<NonMaskable>::handle,
    );
    set_interrupt_handler(
        IDTException::Breakpoint as u64,
        ExceptionHandler::<Breakpoint>::handle,
    );
    set_interrupt_handler(
        IDTException::Overflow as u64,
        ExceptionHandler::<Overflow>::handle,
    );
    set_interrupt_handler(
        IDTException::BoundRangeExceeded as u64,
        ExceptionHandler::<BoundRangeExceeded>::handle,
    );
    set_interrupt_handler(
        IDTException::InvalidOpcode as u64,
        ExceptionHandler::<InvalidOpcode>::handle,
    );
    set_interrupt_handler(
        IDTException::DeviceNotAvailable as u64,
        ExceptionHandler::<DeviceNotAvailable>::handle,
    );
    set_interrupt_handler(
        IDTException::InvalidTSS as u64,
        ExceptionHandler::<InvalidTSS>::handle,
    );
    set_interrupt_handler(
        IDTException::SegmentNotPresent as u64,
        ExceptionHandler::<SegmentNotPresent>::handle,
    );
    set_interrupt_handler(
        IDTException::StackSegmentFault as u64,
        ExceptionHandler::<StackSegmentFault>::handle,
    );
    set_interrupt_handler(
        IDTException::X87FloatingPointException as u64,
        ExceptionHandler::<X87FloatingPointException>::handle,
    );
    set_interrupt_handler(
        IDTException::AlignmentCheck as u64,
        ExceptionHandler::<AlignmentCheck>::handle,
    );
    set_interrupt_handler(
        IDTException::MachineCheck as u64,
        ExceptionHandler::<MachineCheck>::handle,
    );
    set_interrupt_handler(
        IDTException::SIMDFloatingPointException as u64,
        ExceptionHandler::<SIMDFloatingPointException>::handle,
    );
    set_interrupt_handler(
        IDTException::VirtualizationException as u64,
        ExceptionHandler::<VirtualizationException>::handle,
    );
    set_interrupt_handler(
        IDTException::ControlProtection as u64,
        ExceptionHandler::<ControlProtection>::handle,
    );
    set_interrupt_handler(
        IDTException::HypervisorInjection as u64,
        ExceptionHandler::<HypervisorInjection>::handle,
    );
    set_interrupt_handler(
        IDTException::VMMCommunicationException as u64,
        ExceptionHandler::<VMMCommunicationException>::handle,
    );
    set_interrupt_handler(
        IDTException::SecurityException as u64,
        ExceptionHandler::<SecurityException>::handle,
    );

    // Add PS2 Interrupt Handler
    set_interrupt_handler(
        PicInterrupt::Ps2KeyboardInterrupt as u64,
        ps2_keyboard_int_handler,
    );

    // Add the Mouse Interrupt Handler
    set_interrupt_handler(
        PicInterrupt::Ps2MouseInterrupt as u64,
        ps2_mouse_interrupt_handler,
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
