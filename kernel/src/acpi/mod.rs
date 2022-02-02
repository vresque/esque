use core::{mem::size_of, ptr::addr_of, slice::SlicePattern};

use crate::address_of;

#[repr(packed)]
#[derive(Debug)]
pub struct Rsdp2 {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64, // Extended System Descriptor Table Address
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

#[repr(packed)]
#[derive(Debug)]
pub struct SDTHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

#[repr(packed)]
#[derive(Debug)]
pub struct MCFGHeader {
    pub sdt_header: SDTHeader,
    pub reserved: u64,
}

pub fn acpi_structure_from_address_mut<'a, T>(addr: u64) -> &'a mut T {
    unsafe { &mut *(addr as *mut u64 as *mut T) }
}

pub fn acpi_structure_from_address<'a, T>(addr: u64) -> &'a T {
    acpi_structure_from_address_mut(addr)
}

pub fn find_table(sdt: &SDTHeader, signature: &str) -> u64 {
    assert_eq!(signature.len(), 4); // Max Signature Length
    let signature_as_array = signature.as_bytes();
    let entries = (sdt.length - size_of::<SDTHeader>() as u32) / 8;
    for i in 0..entries {
        // This is a terrible cursed creature.
        // Unfortunatley, it does not work with regular arithmatic
        let iter_sdt: &mut SDTHeader = unsafe {
            &mut *(*((address_of!(sdt) + size_of::<SDTHeader>() as u64 + (i as u64 * 8u64))
                as *mut u64) as *mut SDTHeader)
        };
        if iter_sdt.signature.as_slice() == signature_as_array {
            return address_of!(iter_sdt);
        }
    }
    0
}
