use crate::config::handover;
use crate::{
    acpi::{
        acpi_base::{ACPIFindable, ACPITable},
        MCFGHeader, Rsdp2, SDTHeader,
    },
    info, kprint,
    pci::PCI,
};
use bks::Handover;

pub fn init_acpi() {
    let handover = handover();
    info!("Preparing ACPI...");
    let rsdp = Rsdp2::new(handover.rsdp).unwrap();
    let xsdt = SDTHeader::new(rsdp.xsdt_address).unwrap();
    // Print Tables of SDT
    let mcfg = MCFGHeader::find_mut(xsdt).unwrap(); // Equivalent of xsdt.find_table::<MCFGHeader>()

    let pci = PCI::new();
    pci.enumerate(mcfg);
}
