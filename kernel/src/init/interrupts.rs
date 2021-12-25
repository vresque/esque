use bks::Handover;

use crate::drivers::input::ps2::ps2_keyboard_int_handler;
use crate::interrupts::exceptions::ExceptionHandler;
use crate::interrupts::interrupt_frame::InterruptFrame;
use crate::interrupts::set_interrupt_handler;
use crate::memory::paging::page_frame_allocator::request_page;
use crate::pic::PicInterrupt;
use crate::{interrupts::exceptions::IDTException, kprintln};
use core::arch::asm;

use crate::interrupts::idt::{upload_idt, IDTRegister, IDT_REGISTER};

extern "x86-interrupt" fn page_fault_handler(a: InterruptFrame) {
    //panic!("Page Fault detected");
}

extern "x86-interrupt" fn double_fault_handler(a: InterruptFrame) {
    panic!("Double Fault detected");
}

extern "x86-interrupt" fn general_protection_fault_handler(a: InterruptFrame) {
    panic!("General Protection detected");
}

pub fn init_interrupts(_: &mut Handover) {
    kprintln!("Initializing Interrupts");
    let idtr_limit = 0x0FFF;
    let idtr_offset = request_page::<u64>();
    let idtr = IDTRegister::new(idtr_limit, *idtr_offset);
    unsafe {
        IDT_REGISTER.force_unlock();
    }
    IDT_REGISTER.lock().write(idtr);
    kprintln!("{:#?}", IDT_REGISTER.lock().assume_init_mut());

    set_interrupt_handler(IDTException::PageFault as u64, page_fault_handler);
    set_interrupt_handler(IDTException::DoubleFault as u64, double_fault_handler);
    set_interrupt_handler(
        IDTException::GeneralProtectionFault as u64,
        general_protection_fault_handler,
    );

    // Add PS2 Interrupt Handler
    set_interrupt_handler(
        PicInterrupt::KeyboardInterrupt as u64,
        ps2_keyboard_int_handler,
    );

    // Loading the IDT
    kprintln!("Loading IDTR");
    unsafe {
        // Load the IDT
        upload_idt(IDT_REGISTER.lock().assume_init_mut());
    }

    kprintln!("Finished preparing interrupts");
}
