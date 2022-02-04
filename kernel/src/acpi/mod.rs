use core::{mem::size_of, ptr::addr_of, slice::SlicePattern};

use crate::{address_of, impl_acpi_findable};

use self::acpi_base::ACPIFindable;
pub mod acpi_base;
pub mod config;
pub use acpi_base::*;
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

impl_acpi_findable!(Rsdp2 -> "RSDP");

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

impl_acpi_findable!(SDTHeader -> "SDT");

impl SDTHeader {
    pub fn find_table<'retval, 'a, T: ACPIFindable<'a>>(&self) -> Option<&'retval T> {
        T::new::<'retval>(self.find_table_address(T::NAME))
    }

    pub fn find_table_mut<'retval, 'a, T: ACPIFindable<'a>>(&self) -> Option<&'retval mut T> {
        T::new_mut::<'retval>(self.find_table_address(T::NAME))
    }

    pub fn find_table_by_signature<'retval, T>(&self, signature: &str) -> Option<&'retval T> {
        unsafe {
            let ptr = self.find_table_address(signature) as *mut u64 as *mut T;
            if ptr.is_null() {
                None
            } else {
                Some(&*ptr)
            }
        }
    }
    pub fn find_table_by_signature_mut<'retval, T>(
        &self,
        signature: &str,
    ) -> Option<&'retval mut T> {
        unsafe {
            let ptr = self.find_table_address(signature) as *mut u64 as *mut T;
            if ptr.is_null() {
                None
            } else {
                Some(&mut *ptr)
            }
        }
    }

    pub fn find_table_address(&self, signature: &str) -> u64 {
        assert_eq!(signature.len(), 4); // Max Signature Length
        let signature_as_array = signature.as_bytes();
        let entries = (self.length - size_of::<SDTHeader>() as u32) / 8;
        for i in 0..entries {
            // This is a terrible cursed creature.
            // Unfortunatley, it does not work with regular arithmatic
            let iter_sdt: &mut SDTHeader = unsafe {
                &mut *(*((address_of!(self) + size_of::<SDTHeader>() as u64 + (i as u64 * 8u64))
                    as *mut u64) as *mut SDTHeader)
            };
            if iter_sdt.signature.as_slice() == signature_as_array {
                return address_of!(iter_sdt);
            }
        }
        0
    }
}

#[repr(packed)]
#[derive(Debug)]
pub struct MCFGHeader {
    pub sdt_header: SDTHeader,
    pub reserved: u64,
}

impl_acpi_findable!(MCFGHeader -> "MCFG");
