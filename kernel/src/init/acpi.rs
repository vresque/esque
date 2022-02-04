use core::mem::size_of;

use crate::{
    acpi::{acpi_base::ACPIFindable, MCFGHeader, Rsdp2, SDTHeader},
    debug, info, kprint,
};
use bks::Handover;

pub fn init_acpi(handover: &mut Handover) {
    info!("Preparing ACPI...");
    let rsdp = Rsdp2::new(handover.rsdp).unwrap();
    let xsdt = SDTHeader::new(rsdp.xsdt_address).unwrap();
    // Print Tables of SDT
    let entries = (xsdt.length - size_of::<SDTHeader>() as u32) / 8;
    let mcfg = MCFGHeader::find(xsdt).unwrap(); // Equivalent of xsdt.find_table::<MCFGHeader>()
    debug!("{:?}", mcfg);
}
