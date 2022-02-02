#![no_std]

use core::slice;

use enumtastic::enum_with_options;
pub const PAGE_SIZE: u64 = 0x1000; // 4096

enum_with_options! {
    pub enum MemoryType: u32 => {
        ReservedMemory          = 0,
        LoaderCode              = 1,
        LoaderData              = 2,
        BootServicesCode        = 3,
        BootServicesData        = 4,
        RuntimeServicesCode     = 5,
        RuntimeServicesData     = 6,
        ConventialMemory        = 7,
        UnusableMemory          = 8,
        ACPIReclaimMemory       = 9,
        ACPIMemoryNVS           = 10,
        MemoryMappedIO          = 11,
        MemoryMappedIOPortSpace = 12,
        PalCode                 = 13,
        PersistentMemory        = 14,
        EmptyTemporaryMemory    = 254,
    }
}

impl MemoryType {
    pub fn custom(val: u32) -> Self {
        Self(val)
    }
}

// From https://github.com/rust-osdev/uefi-rs/blob/master/src/table/boot.rs#L1072-L1099
bitflags::bitflags! {
    #[allow(non_upper_case_globals)]
    pub struct MemoryAttribute: u64 {
        const UNREACHABLE           = 0x1;
        const WRITE_COMBINE          = 0x2;
        const WRITE_THROUGH          = 0x4;
        const WRITE_BACK             = 0x8;
        const UNREACHABLE_EXPORTED   = 0x10;
        const WRITE_PROTECT          = 0x1000;
        const READ_PROTECT           = 0x2000;
        const EXECUTE_PROTECT        = 0x4000;
        const NON_VOLATILE           = 0x8000;
        const MORE_READABLE          = 0x10000;
        const READ_ONLY              = 0x20000;
        const RUNTIME               = 0x8000_0000_0000_000;
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub ty: MemoryType,
    // So that we can mem::transmute the other descriptor
    padding: u32,
    pub phys_base: u64,
    pub virt_base: u64,
    pub page_count: u64,
    pub attributes: MemoryAttribute,
}

impl EfiMemoryDescriptor {
    pub fn empty() -> Self {
        Self {
            ty: MemoryType::EmptyTemporaryMemory,
            padding: 0,
            phys_base: 0,
            virt_base: 0,
            page_count: 0,
            attributes: MemoryAttribute::UNREACHABLE,
        }
    }

    pub fn new(
        pages: u64,
        ty: MemoryType,
        attr: MemoryAttribute,
        phys_base: u64,
        virt_base: u64,
    ) -> Self {
        Self {
            ty,
            padding: 0,
            phys_base,
            virt_base,
            page_count: pages,
            attributes: attr,
        }
    }
}

impl core::fmt::Debug for EfiMemoryDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EfiMemoryDescriptor\n")?;
        write!(f, "\t::Type: {:#x?}\n", self.ty)?;
        write!(f, "\t::Base: {:#8x?}\n", self.phys_base)?;
        write!(f, "\t::Virt: {:#8x?}\n", self.virt_base)?;
        write!(f, "\t::Pages: {:#x?}\n", self.page_count)?;
        write!(f, "\t::Attrs: {:#?}\n", self.attributes)?;
        Ok(())
    }
}

#[repr(C)]
pub struct Handover {
    // Must always be 42: If not, a bad bootloader was used
    checknum: u32,
    framebuffer: Framebuffer,
    font: Psf1Font,
    #[allow(unused)]
    pub mmap_size: usize,
    memory_map: *mut EfiMemoryDescriptor,
    pub mmap_entries: usize,
    pub mmap_entry_size: usize,
    pub config: Config,
    pub initramfs_base: u64,
    pub initramfs_size: usize,
    pub rsdp: u64,
}

impl Handover {
    pub fn new(
        fb: Framebuffer,
        font: Psf1Font,
        mmap: *mut EfiMemoryDescriptor,
        mmap_size: usize,
        mmap_entry_size: usize,
        mmap_entries: usize,
        config: Config,
        initramfs_base: u64,
        initramfs_size: usize,
    ) -> Self {
        Self {
            checknum: 42,
            framebuffer: fb,
            font: font,
            memory_map: mmap,
            mmap_size: mmap_size,
            mmap_entries: mmap_entries,
            mmap_entry_size: mmap_entry_size,
            config: config,
            initramfs_base,
            initramfs_size,
            rsdp: 0,
        }
    }

    pub fn checknum(&self) -> u32 {
        self.checknum
    }

    pub fn framebuffer(&mut self) -> &mut Framebuffer {
        &mut self.framebuffer
    }

    pub fn font(&mut self) -> &mut Psf1Font {
        &mut self.font
    }

    pub fn memory_map(&mut self) -> &[EfiMemoryDescriptor] {
        unsafe { self.retrieve_memory_map() }
    }

    pub fn memory_map_mut(&mut self) -> &mut [EfiMemoryDescriptor] {
        unsafe { self.retrieve_memory_map() }
    }

    pub fn raw_memory_map(&mut self) -> *mut EfiMemoryDescriptor {
        self.memory_map
    }

    unsafe fn retrieve_memory_map(&mut self) -> &mut [EfiMemoryDescriptor] {
        core::slice::from_raw_parts_mut(self.memory_map, self.mmap_entries)
    }

    pub fn move_initramfs_to(&mut self, addr: u64) {
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.initramfs_base as *const u64,
                addr as *mut u64,
                self.initramfs_size,
            );
        }
    }

    pub fn initramfs(&mut self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.initramfs_base as *mut u8, self.initramfs_size) }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Psf1Header {
    pub magic: [u8; 2],
    pub mode: u8,
    pub charsize: u8,
}

impl Psf1Header {
    pub fn charsize(&mut self) -> u8 {
        self.charsize
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Psf1Font {
    pub header: Psf1Header,
    pub buffer: u32,
    pub size: usize,
}

impl Psf1Font {
    pub fn new(header: Psf1Header, buffer: *mut u8, size: usize) -> Self {
        let cast = buffer as u32;
        Self {
            header,
            buffer: cast,
            size,
        }
    }

    pub fn header(&mut self) -> &mut Psf1Header {
        &mut self.header
    }

    pub fn buffer(&self) -> &[u8] {
        unsafe { self.retrieve_buffer() }
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        unsafe { self.retrieve_buffer() }
    }

    unsafe fn retrieve_buffer<'a>(&self) -> &'a mut [u8] {
        slice::from_raw_parts_mut(self.buffer as *mut u8, self.size)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Framebuffer {
    pub base: u64,
    pub size: usize,
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

impl Framebuffer {
    pub fn new(base: u64, size: usize, width: usize, height: usize, stride: usize) -> Self {
        Self {
            base,
            size,
            width,
            height,
            stride,
        }
    }

    pub fn raw_buffer(&mut self) -> *mut u8 {
        self.base as *mut u8
    }

    pub fn buffer(&self) -> &[u8] {
        unsafe { self.retrieve_buffer() }
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        unsafe { self.retrieve_buffer() }
    }

    pub fn clear_bcolor(&mut self, color: u32) {
        for byte in self.buffer_mut() {
            // This upcast allows colours up to u32::MAX
            // Without this, a u8 will be used which only allows
            // 256 colours.
            unsafe { *(byte as *mut u8) = color as u8 };
        }
    }

    unsafe fn retrieve_buffer<'a>(&self) -> &'a mut [u8] {
        slice::from_raw_parts_mut(self.base as *mut u8, self.size)
    }
}

impl core::fmt::Display for Framebuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Framebuffer Information: ")?;
        writeln!(f, "   Base Address: {:#x?}", self.base)?;
        writeln!(f, "   Size: {:#x}", self.size)?;
        writeln!(f, "   Width: {}", self.width)?;
        writeln!(f, "   Height: {}", self.height)?;
        writeln!(f, "   Stride: {}", self.stride)?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Config {
    pub lang: u16,
    pub layout: u16,
}

impl Config {
    pub fn new(lang: u16, layout: u16) -> Self {
        Self { lang, layout }
    }

    pub const fn default() -> Self {
        Self { lang: 0, layout: 0 }
    }
}

pub const LANGUAGES_SUPPORTED_NUM: usize = 2;
enumtastic::const_enum! {
    pub enum Language: u16 => {
        English = 0,
        German  = 1,
    }

    impl {}
}

pub const KEYBOARD_LAYOUTS_SUPPORTED_NUM: usize = 2;
enumtastic::const_enum! {
    pub enum KeyboardLayout: u16 => {
        QWERTY = 0,
        German = 1,
    }

    impl {}
}
