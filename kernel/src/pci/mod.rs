use core::mem::size_of;

use crate::{
    acpi::{config::DeviceConfig, ACPITable, MCFGHeader},
    address_of,
};

pub struct PCI {}

impl PCI {
    pub fn new() -> Self {
        Self {}
    }
    pub fn enumerate(&self, mcfg: &mut MCFGHeader) {
        let mcfg_entry_count = (mcfg.sdt_header.length - size_of::<MCFGHeader>() as u32)
            / size_of::<DeviceConfig>() as u32;
        for i in 0..mcfg_entry_count {
            let config = DeviceConfig::new(
                address_of!(mcfg) // At MCFG
                    + size_of::<MCFGHeader>() as u64 // Offset by the MCFGHeader
                    + (size_of::<DeviceConfig>() as u64 * i as u64), // Skip the already iterated ones,
            )
            .unwrap();

            // Iterate over all of the buses
            for bus in config.start_bus..config.end_bus {
                self.enumerate_bus(config.base, bus as i64);
            }
        }
    }

    fn enumerate_bus(&self, base: u64, bus: i64) {
        let offset = bus << 20;
        let _address = base as i64 + offset; //  The Address of the bus
    }
    fn enumerate_device(&self, _base: u64, _device: u64) {}

    fn enumerate_function(&self, _device_addr: u64, _func: u64) {}
}
