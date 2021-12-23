use bks::Handover;

use crate::interrupts::exceptions::{ExceptionHandler, IDTException};
use crate::interrupts::set_interrupt_handler;
use crate::kprintln;
use crate::{
    interrupts::idt::IDTDescriptorEntry, memory::paging::page_frame_allocator::request_page,
};
use core::arch::asm;

use crate::interrupts::idt::{IDTRegister, IDT_REGISTER};

pub fn init_interrupts(handover: &mut Handover) {
    kprintln!("Initializing Interrupts");
    let idtr_limit = 0x0FFF;
    let idtr_offset = request_page::<u64>();
    let idtr = IDTRegister::new(idtr_limit, *idtr_offset);
    IDT_REGISTER.lock().write(idtr);

    set_interrupt_handler(
        IDTException::PageFault,
        ExceptionHandler::<{ IDTException::PageFault }>::handle,
    );

    // Loading the IDT
    unsafe {
        asm!("lidt {}", in(reg) (IDT_REGISTER.lock().assume_init_mut() as *mut IDTRegister));
    }
}
