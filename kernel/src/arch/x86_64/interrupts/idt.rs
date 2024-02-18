use core::mem::MaybeUninit;

use spin::Mutex;

use super::interrupt_frame::InterruptFrame;

// https://wiki.osdev.org/Interrupt_Descriptor_Table
#[repr(u8)]
#[allow(unused)]
pub enum IDTTypesAndAttrs {
    InterruptGate = 0b10001110,
    CallGate = 0b10001100,
    TrapGate = 0b10001111,
}

impl Into<u8> for IDTTypesAndAttrs {
    fn into(self) -> u8 {
        return self as u8;
    }
}

// https://wiki.osdev.org/Interrupt_Descriptor_Table
#[repr(C, packed)]
pub struct IDTDescriptorEntry {
    offset_0: u16,
    segment_selector: u16,
    interrupt_stack_table_offset: u8,
    type_and_attrs: u8,
    offset_1: u16,
    offset_2: u32,
    _padding: u32, // Useless
}

impl IDTDescriptorEntry {
    pub fn new(offset: u64, type_and_attrs: u8, segment_selector: u16) -> Self {
        let mut default = Self {
            offset_0: 0,
            segment_selector: 0,
            interrupt_stack_table_offset: 0,
            type_and_attrs: 0,
            offset_1: 0,
            offset_2: 0,
            _padding: 0,
        };
        default.set_offset(offset);
        default.type_and_attrs = type_and_attrs;
        default.segment_selector = segment_selector;
        default
    }

    pub fn with_function(
        func: extern "x86-interrupt" fn(InterruptFrame),
        type_and_attrs: u8,
        segment_selector: u16,
    ) -> Self {
        let as_u64 = func as u64;
        Self::new(as_u64, type_and_attrs, segment_selector)
    }

    /// Sets the entire offset using a single u64
    pub fn set_offset(&mut self, offset: u64) {
        self.offset_0 = (  offset & 0x000000000000ffff) as u16 /* Get Lo */;
        self.offset_1 = ( (offset & 0x00000000ffff0000) >> 16 ) as u16 /* Get Hi */;
        self.offset_2 = ((offset & 0xffffffff00000000) >> 16) as u32;
    }

    pub fn get_offset(&self) -> u64 {
        let mut offset = 0;
        offset |= (self.offset_0) as u64;
        offset |= (self.offset_1 as u64) << (16 as u64);
        offset |= (self.offset_2 as u64) << (32 as u64);
        offset
    }
}

// https://wiki.osdev.org/Interrupt_Descriptor_Table
#[repr(packed, C)]
#[derive(Copy, Clone, Debug)]
pub struct IDTRegister {
    limit: u16,
    offset: u64,
}
impl IDTRegister {
    #[inline]
    pub fn new(limit: u16, offset: u64) -> Self {
        Self { limit, offset }
    }
}

pub fn upload_idt_entry_at(offset: u64, value: IDTDescriptorEntry) {
    let idt_entry = unsafe {
        &mut *((IDT_REGISTER.lock().assume_init_mut().offset
            + (offset * core::mem::size_of::<IDTDescriptorEntry>() as u64))
            as *mut u64 as *mut IDTDescriptorEntry)
    };

    *idt_entry = value;
}

pub static IDT_REGISTER: Mutex<MaybeUninit<IDTRegister>> = Mutex::new(MaybeUninit::uninit());

#[inline(always)]
pub unsafe fn upload_idt(register: &IDTRegister) {
    core::arch::asm!("lidt [{}]", in(reg) register, options(nostack));
}
