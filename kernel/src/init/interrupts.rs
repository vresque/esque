use bks::Handover;

use crate::{
    interrupts::idt::IDTDescriptorEntry, memory::paging::page_frame_allocator::request_page,
};

use crate::interrupts::idt::IDTRegister;

pub fn init_interrupts(handover: &mut Handover) {
    let idtr_limit = 0x0FFF;
    let idtr_offset = request_page::<u64>();
    let idtr = IDTRegister::new(idtr_limit, *idtr_offset);

    let page_fault = 
}
