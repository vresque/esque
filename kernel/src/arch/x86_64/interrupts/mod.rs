use self::{
    idt::{upload_idt_entry_at, IDTDescriptorEntry, IDTTypesAndAttrs},
    interrupt_frame::InterruptFrame,
};

pub mod exceptions;
pub mod idt;
pub mod interrupt_frame;
pub mod register;

pub fn set_interrupt_handler(offset: u64, handler: extern "x86-interrupt" fn(InterruptFrame)) {
    let idt_desc =
        IDTDescriptorEntry::with_function(handler, IDTTypesAndAttrs::InterruptGate as u8, 0x08);
    upload_idt_entry_at(offset, idt_desc)
}
