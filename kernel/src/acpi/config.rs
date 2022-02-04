use super::ACPITable;
use crate::impl_acpi_table;

#[repr(packed)]
pub struct DeviceConfig {
    pub base: u64,
    pub pci_seg_group: u16,
    pub start_bus: u8,
    pub end_bus: u8,
    _reserved: u32,
}

impl_acpi_table!(DeviceConfig);
