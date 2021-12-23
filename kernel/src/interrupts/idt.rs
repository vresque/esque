use core::mem::MaybeUninit;

use spin::Mutex;

// https://wiki.osdev.org/Interrupt_Descriptor_Table
#[repr(u8)]
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
#[repr(C)]
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
    /// Sets the entire offset using a single u64
    pub fn set_offset(&mut self, offset: u64) {
        self.offset_0 = (  offset & 0x000000000000ffff) as u16 /* Get Lo */;
        self.offset_1 = ( (offset & 0x00000000ffff0000) >> 16 ) as u16 /* Get Hi */;
        self.offset_2 = ((offset & 0xffffffff00000000) >> 16) as u32;
    }

    pub fn get_offset(&self) -> u64 {
        let offset = 0;
        offset |= (self.offset_0) as u64;
        offset |= (self.offset_1 << 16) as u64;
        offset |= (self.offset_2 << 32) as u64;
        offset
    }
}

// https://wiki.osdev.org/Interrupt_Descriptor_Table
#[repr(packed, C)]
pub struct IDTRegister {
    limit: u16,
    offset: u64,
}
impl IDTRegister {
    pub fn new(limit: u16, offset: u64) -> Self {
        Self { limit, offset }
    }
}

pub fn upload_idt_entry_at(offset: u64, value: IDTDescriptorEntry) {
    let idt_entry = unsafe {
        &mut *((IDT_REGISTER.lock().assume_init_mut().offset + offset) as *mut u64
            as *mut IDTDescriptorEntry)
    };

    *idt_entry = value;
}

static mut IDT_REGISTER: Mutex<MaybeUninit<IDTRegister>> = Mutex::new(MaybeUninit::uninit());
