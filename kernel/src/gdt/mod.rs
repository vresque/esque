#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ring {
    Ring0 = 0b00,
    Ring3 = 0b11,
}

enumtastic::const_enum! {
    pub enum GdtEntryType: u16 => {
        KernelCode = 1,
        KernelData = 2,
        KernelTls = 3,
        UserCode32Unused = 4,
        Tss = 8,
        Tss_Hi = 9,
    }

    impl {}
}

#[repr(packed)]
#[repr(C)]
pub struct GDTDescriptor {
    size: u16,
    offset: u64,
}

impl GDTDescriptor {
    pub fn new(size: u16, offset: u64) -> Self {
        Self { size, offset }
    }
}

#[repr(packed)]
#[repr(C)]
pub struct GDTEntry {
    limit_0: u16,
    base_0: u16,
    base_1: u8,
    read_write: u8,
    limit_1_flags: u8,
    base_2: u8,
}

impl GDTEntry {
    pub const fn new(
        limit_0: u16,
        base_0: u16,
        base_1: u8,
        read_write: u8,
        limit_1_flags: u8,
        base_2: u8,
    ) -> Self {
        Self {
            limit_0,
            base_0,
            base_1,
            read_write,
            limit_1_flags,
            base_2,
        }
    }
}

#[repr(packed(0x1000))]
#[repr(C)]
pub struct GlobalDescriptorTable {
    null: GDTEntry,        // + 0x0
    kernel_code: GDTEntry, // + 0x08
    kernel_data: GDTEntry, // + 0x10
    user_null: GDTEntry,
    user_code: GDTEntry,
    user_data: GDTEntry,
}

impl GlobalDescriptorTable {
    pub const fn new() -> Self {
        let null = GDTEntry::new(0, 0, 0, 0, 0, 0);
        let kernel_code = GDTEntry::new(0, 0, 0, 0x9a, 0xa0, 0);
        let kernel_data = GDTEntry::new(0, 0, 0, 0x92, 0xa0, 0);

        let user_null = GDTEntry::new(0, 0, 0, 0x00, 0x00, 0);
        let user_code = GDTEntry::new(0, 0, 0, 0x9a, 0xa0, 0);
        let user_data = GDTEntry::new(0, 0, 0, 0x92, 0xa0, 0);

        Self {
            null,
            kernel_code,
            kernel_data,
            user_null,
            user_code,
            user_data,
        }
    }
}

pub static GLOBAL_GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

// Load gdt.s assembly file
core::arch::global_asm!(include_str!("gdt.s"));
extern "C" {
    pub fn upload_gdt(gdt_desc: *mut GDTDescriptor);
}
