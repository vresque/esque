use core::arch::asm;

pub unsafe fn upload_gdt(gdt: *mut GDTDescriptor) {
    asm!("lgdt [{}]", in(reg) gdt);
}

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
        UserData = 5,
        UserCode = 6,
        Tss = 7,
        TssHigh = 9,
    }

    impl {}
}

enumtastic::const_enum! {
    pub enum GdtAccess: u8 => {
        Present = 1 << 7,
        Ring0 = 0 << 5,
        Ring1 = 1 << 5,
        Ring2 = 2 << 5,
        Ring3 = 3 << 5,
        System = 1 << 4,
        Executable = 1 << 3,
        Conforming = 1 << 2,
        Privilege = 1 << 1,
        Dirty = 1,
        TssAvailable = 0x9,
        TssBusy = 0xB,
    }

    impl {}
}

enumtastic::const_enum! {
    pub enum GdtFlag: u8 => {
        PageSize = 1 << 7,
        ProtectedMode = 1 << 6,
        LongMode = 1 << 5,
    }

    impl {}
}

#[repr(packed)]
#[repr(C)]
pub struct GDTDescriptor {
    size: u16,
    offset: *const u64,
}

impl GDTDescriptor {
    pub fn new(size: u16, offset: *const u64) -> Self {
        Self { size, offset }
    }
}

#[repr(packed)]
#[repr(C)]
pub struct GDTEntry {
    limit_0: u16,
    base_0: u16,
    base_1: u8,
    access: u8,
    limit_1_flags: u8,
    base_2: u8,
}

impl GDTEntry {
    pub const fn new(
        limit_0: u16,
        base_0: u16,
        base_1: u8,
        access: u8,
        limit_1_flags: u8,
        base_2: u8,
    ) -> Self {
        Self {
            limit_0,
            base_0,
            base_1,
            access,
            limit_1_flags,
            base_2,
        }
    }

    pub const fn new32(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        Self::new(
            limit as u16,
            base as u16,
            (base >> 16) as u8,
            access,
            flags & 0xF0 | ((limit >> 16) as u8) & 0x0F,
            (base >> 24) as u8,
        )
    }

    pub fn set_base(&mut self, base_0: u16, base_1: u8, base_2: u8) {
        self.base_0 = base_0;
        self.base_1 = base_1;
        self.base_2 = base_2;
    }

    pub fn set_base32(&mut self, base: u32) {
        self.set_base(base as u16, (base >> 16) as u8, (base >> 24) as u8);
    }

    pub fn set_limit32(&mut self, limit: u32) {
        self.set_limit(
            limit as u16,
            self.limit_1_flags & 0xF0 | ((limit >> 16) as u8) & 0x0F,
        )
    }

    pub fn set_limit(&mut self, limit_0: u16, limit_1_flags: u8) {
        self.limit_0 = limit_0;
        self.limit_1_flags = limit_1_flags;
    }
}

#[repr(packed(0x1000))]
#[repr(C)]
pub struct GlobalDescriptorTable {
    null: GDTEntry,        // + 0x0
    kernel_code: GDTEntry, // + 0x08
    kernel_data: GDTEntry, // + 0x10
    user_dummy: GDTEntry,
    user_data: GDTEntry,
    user_code: GDTEntry,
    tsslo: GDTEntry, // Must be 16 bits
    tsshi: GDTEntry, // Must be 16 bits
}

impl GlobalDescriptorTable {
    pub const fn new() -> Self {
        let null = GDTEntry::new32(0, 0, 0, 0);
        let kernel_code = GDTEntry::new32(
            0,
            0,
            GdtAccess::Present
                | GdtAccess::Ring0
                | GdtAccess::System
                | GdtAccess::Executable
                | GdtAccess::Privilege,
            GdtFlag::LongMode,
        );
        let kernel_data = GDTEntry::new32(
            0,
            0xFFFFF,
            GdtAccess::Present | GdtAccess::Ring0 | GdtAccess::System | GdtAccess::Privilege,
            GdtFlag::LongMode,
        );

        let user_dummy = GDTEntry::new32(
            0,
            0,
            GdtAccess::Present
                | GdtAccess::Ring0
                | GdtAccess::System
                | GdtAccess::Privilege
                | GdtAccess::Executable,
            GdtFlag::LongMode,
        );
        let user_data = GDTEntry::new32(
            0,
            0,
            GdtAccess::Present | GdtAccess::Ring3 | GdtAccess::System | GdtAccess::Privilege,
            GdtFlag::LongMode,
        );
        let user_code = GDTEntry::new32(
            0,
            0,
            GdtAccess::Present
                | GdtAccess::Ring3
                | GdtAccess::System
                | GdtAccess::Privilege
                | GdtAccess::Executable,
            GdtFlag::LongMode,
        );

        // TSSlo
        let tsslo = GDTEntry::new32(
            0,
            0,
            GdtAccess::Present | GdtAccess::Ring3 | GdtAccess::TssAvailable,
            0,
        );
        let tsshi = GDTEntry::new32(0, 0, 0, 0);
        let reserved = GDTEntry::new32(0, 0, 0, 0);

        Self {
            null,
            kernel_code,
            kernel_data,
            user_dummy,
            user_data,
            user_code,
            tsslo,
            tsshi,
        }
    }

    pub fn as_ptr(&self) -> *const u64 {
        self as *const Self as *const u64
    }

    pub fn get_entry(&mut self, idx: usize) -> &mut GDTEntry {
        unsafe {
            &mut *((self as *mut Self as *mut GDTEntry).add(idx * core::mem::size_of::<GDTEntry>()))
        }
    }
}

#[repr(packed(0x1000))]
#[repr(C)]
pub struct InitialGlobalDescriptorTable {
    null: GDTEntry,        // + 0x0
    kernel_code: GDTEntry, // + 0x08
    kernel_data: GDTEntry, // + 0x10
}

impl InitialGlobalDescriptorTable {
    pub const fn new() -> Self {
        let null = GDTEntry::new32(0, 0, 0, 0);
        let kernel_code = GDTEntry::new32(
            0,
            0,
            GdtAccess::Present
                | GdtAccess::Ring0
                | GdtAccess::System
                | GdtAccess::Executable
                | GdtAccess::Privilege,
            GdtFlag::LongMode,
        );
        let kernel_data = GDTEntry::new32(
            0,
            0,
            GdtAccess::Present | GdtAccess::Ring0 | GdtAccess::System | GdtAccess::Privilege,
            GdtFlag::LongMode,
        );

        Self {
            null,
            kernel_code,
            kernel_data,
        }
    }

    pub fn as_ptr(&self) -> *const u64 {
        self as *const Self as *const u64
    }
}

pub static INITIAL_KERNEL_GDT: InitialGlobalDescriptorTable = InitialGlobalDescriptorTable::new();

#[thread_local]
pub static GLOBAL_GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();
