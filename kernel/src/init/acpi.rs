use core::mem::size_of;

use crate::{acpi::*, debug, info, kprint};
use bks::Handover;

pub fn init_acpi(handover: &mut Handover) {
    info!("Preparing ACPI...");
    let rsdp: &mut Rsdp2 = acpi_structure_from_address_mut(handover.rsdp);
    let xsdt: &mut SDTHeader = acpi_structure_from_address_mut(rsdp.xsdt_address);
    // Print Tables of SDT
    let entries = (xsdt.length - size_of::<SDTHeader>() as u32) / 8;
    let mcfg: &mut MCFGHeader = acpi_structure_from_address_mut(find_table(xsdt, "MCFG"));
}
