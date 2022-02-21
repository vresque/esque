use core::mem::size_of;

use pci_lookup::{
    get_device_name, get_prog_if_name, get_subclass_name, get_vendor_name, DEVICE_CLASSES,
};

use crate::{
    acpi::{config::DeviceConfig, ACPITable, MCFGHeader},
    address_of, from_addr,
    memory::paging::page_table_manager::PAGE_TABLE_MANAGER,
};

struct PCIDeviceHeader {
    pub vendor_id: u16,
    pub device_id: u16,
    pub command: u16,
    pub status: u16,
    pub revision_id: u8,
    pub program_interface: u8,
    pub subclass: u8,
    pub class: u8,
    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub bist: u8,
}

impl core::fmt::Display for PCIDeviceHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f)?;
        writeln!(f, "PCIDeviceHeader:")?;
        writeln!(
            f,
            "   (Vendor ID = {:#x?}; Device ID = {:#x?}; Program Interface = {:#x?})",
            self.vendor_id, self.device_id, self.program_interface
        )?;
        writeln!(
            f,
            "   Vendor = {}; Device = {}; Class = {}; Subclass = {}; Program Interface = {}",
            get_vendor_name(self.vendor_id),
            get_device_name(self.vendor_id, self.device_id),
            DEVICE_CLASSES[self.class as usize],
            get_subclass_name(self.class, self.subclass),
            get_prog_if_name(self.class, self.subclass, self.program_interface)
        )?;

        Ok(())
    }
}

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
                self.enumerate_bus(config.base, bus as u64);
            }
        }
    }

    fn enumerate_bus(&self, base: u64, bus: u64) {
        let offset = bus << 20;
        let address = base + offset; //  The Address of the bus
        unsafe {
            PAGE_TABLE_MANAGER
                .lock()
                .assume_init_mut()
                .map_memory(address as u64, address as u64);
        }

        let header: &PCIDeviceHeader = from_addr!(address);

        // Check for invalid Device IDs
        if header.device_id == 0 || header.device_id == 0xFFFF {
            return;
        }

        // 32 Devices per Bus
        for device in 0..32 {
            self.enumerate_device(address, device)
        }
    }
    fn enumerate_device(&self, bus_address: u64, device: u64) {
        let offset = device << 15;
        let address = bus_address + offset; //  The Address of the bus
        unsafe {
            PAGE_TABLE_MANAGER
                .lock()
                .assume_init_mut()
                .map_memory(address as u64, address as u64);
        }

        let header: &PCIDeviceHeader = from_addr!(address);

        // Check for invalid Device IDs
        if header.device_id == 0 || header.device_id == 0xFFFF {
            return;
        }
        // 8 Functions per Device
        for function in 0..8 {
            self.enumerate_function(address, function)
        }
    }

    fn enumerate_function(&self, device_addr: u64, func: u64) {
        let offset = func << 12;
        let address = device_addr + offset; //  The Address of the bus
        unsafe {
            PAGE_TABLE_MANAGER
                .lock()
                .assume_init_mut()
                .map_memory(address as u64, address as u64);
        }

        let header: &PCIDeviceHeader = from_addr!(address);

        // Check for invalid Device IDs
        if header.device_id == 0 || header.device_id == 0xFFFF {
            return;
        }

        // Print all PCI Devices
        //debug!("{}", header);
    }
}
