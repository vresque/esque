#![no_std]

//! All of these informations stem from wiki.osdev.org/PCI
//! This might seem useless, yet it will be useful when debugging
//! the kernel from userspace. All of this 'stringified' information
//! will be written to a FileDescriptor

use core::fmt::Debug;

pub const DEVICE_CLASSES: [&str; 20] = [
    "Unclassified",
    "Mass Storage Controller",
    "Network Controller",
    "Display Controller",
    "Multimedia Controller",
    "Memory Controller",
    "Bridge Device",
    "Simple Communication Controller",
    "Base System Peripheral",
    "Input Device Controller",
    "Docking Station",
    "Processor",
    "Serial Bus Controller",
    "Wireless Controller",
    "Intelligent Controller",
    "Satellite Communicatin Controller",
    "Encryption Controller",
    "Signal Processing Controller",
    "Processing Accelerator",
    "Non Essential Instrumentation",
];

/// # Get Vendor Name
/// Gets the vendor name
/// ## ATTENTION
/// *Not all vendors are listed here*
/// There exist thousands (I presume) of vendors, trying
/// to cover them all would be an impossible feat. (There are 47k+ PCI Devices)
/// ## Source
/// All of those names are from the amazing website https://www.pcilookup.com
pub fn get_vendor_name(id: u16) -> &'static str {
    match id {
        0x8086 => "Intel Corporation",
        0x1022 => "AMD",
        0x1dDE => "NVIDIA Corporation",
        0x1630 => "2Wire, Inc.",
        0xA727 | 0x6891 => "3Com",
        0x12b9 => "3Com Corp, Modem Division",
        0x0506 => "3Com Corp.",
        0x10b7 => "3Com Corporation",
        0x1234 => "QEMU",
        _ => "Unidentified Vendor",
    }
}

/// # Get Device Name
/// Gets a devices name depending on the vendor id
/// ## ATTENTION
/// *This is merely a debugging function. There exist over 47 thousand
/// PCI devices (just on pcilookup), all with different names. I will never support them all.
pub fn get_device_name(vendor: u16, device: u16) -> &'static str {
    match vendor {
        0x8086 /* Intel */ => {
            match device {
                // The following are just the QEMU default devices
                0x29c0 => "Express DRAM Controller",
                0x2918 => "LPC Interface Controller",
                0x2922 => "6 port SATA Controller [AHCI mode]",
                0x2930 => "SMBus Controller",
                0x10d3 => "Gigabit CT Desktop Adapter",
                _ => "Unknown Intel Device"
            }
        }
        0x1234 => "QEMU Specific PCI Device",
        _ => "Unknown Device"
    }
}

macro_rules! match_tree {
    (
        major = $major_matcher:expr => minor = $minor_matcher:expr =>
        config: { major-default = $final_major:expr ; minor-default = $final_minor:expr } => {$(
            $major_match:tt {
                $(
                    $minor_match:tt = $minor_result:expr,
                )*
            },
        )*
    }
    ) => {
        match $major_matcher {
            $(
                $major_match => {
                    match $minor_matcher {
                        $(
                            $minor_match => { $minor_result },
                        )*
                        _ => { $final_minor }
                    }
                },
            )*
            _ => { $final_major }
        }
    };
}

/// # Get Subclass Name
/// Gets a subclasses name depending on the vendor id and the device ID
pub fn get_subclass_name(class: u8, subclass: u8) -> &'static str {
    match_tree! {
        major = class => minor = subclass =>
        config: { major-default = "Unknown Class"; minor-default = "Unknown Subclass" } =>
        {
            /* Unclassified */
            0x0  {
                0x0 = "Non-VGA-Compatible Unclassified Device",
                0x1 = "VGA-Compatible Unclassified Device",
            },
            /* Mass Storage Controller */
            0x1 {
                0x1 = "IDE Controller",
                0x2 = "Floppy Disk Controller",
                0x3 = "IPIP Bus Controller",
                0x4 = "RAID Controller",
                0x5 = "ATA Controller",
                0x6 = "Serial ATA Controller",
                0x7 = "Serial Attached SCSI Controller",
                0x8 = "Non-Volatile Memory Controller",
                0x80 = "Other",
            },
            /* Network Controller */
            0x2 {
                0x0 = "Ethernet Controller",
                0x1 = "Token Ring Controller",
                0x2 = "FDDI Controller",
                0x3 = "ATM Controller",
                0x4 = "ISDN Controller",
                0x5 = "WorldFlip Controller",
                0x6 = "PICMG 2.14 Multi Computing Controller",
                0x7 = "Infiniband Controller",
                0x8 = "Fabric Controller",
                0x80 = "Other",
            },
            /* Display Controller */
            0x3 {
                0x0 = "VGA Compatible Controller",
                0x1 = "XGA Controller",
                0x2 = "3D Controller (Not VGA-Compatible)",
                0x80 = "Other",
            },
            0x4 {
                0x0 = "Multimedia Video Controller",
                0x1 = "Multimedia Audio Controller",
                0x2 = "Computer Telephony Device",
                0x3 = "Audio Device",
                0x80 = "Other",
            },
            /* Memory Controller */
            0x5 {
                0x0 = "RAM Controller",
                0x1 = "Flash Controller",
                0x80 = "Other",
            },
            /* Bridge */
            0x6 {
                0x0 = "Host Bridge",
                0x1 = "ISA Bridge",
                0x2 = "EISA Bridge",
                0x3 = "MCA Bridge",
                0x4 = "PCI-to-PCI Bridge",
                0x5 = "PCMCIA Bridge",
                0x6 = "NuBus Bridge",
                0x7 = "CardBus Bridge",
                0x8 = "RACEway Bridge",
                0x9 = "PCI-to-PCI Bridge",
                0x0A = "InfiniBand-to-PCI Host Bridge",
                0x80 = "Other",
            },
            /* Simple Communication Controller */
            0x7 {
                0x0 = "Serial Controller",
                0x1 = "Parallel Controller",
                0x2 = "Multiport Serial Controller",
                0x3 = "Modem",
                0x4 = "IEEE 488.1/2 (GPIB) Controller",
                0x5 = "Smart Card Controller",
                0x80 = "Other",
            },
            /* Base System Peripheral */
            0x8 {
                0x0 = "PIC",
                0x01 = "DMA Controller",
                0x02 = "Timer",
                0x3 = "RTC Controller",
                0x4 = "PCI Hot-Plug Controller",
                0x5 = "SD Host controller",
                0x6 = "IOMMU",
                0x80 = "Other",
            },
            /* Input Device Controller */
            0x9 {
                0x0 = "Keyboard Controller",
                0x1 = "Digitizer Pen",
                0x2 = "Mouse Controller",
                0x3 = "Scanner Controller",
                0x4 = "Gameport Controller",
                0x80 = "Other",
            },
            /* Docking Station */
            0xA {
                0x0 = "Generic",
                0x80 = "Other",
            },
            /* Processor */
            0xB {
                0x0 = "386",
                0x1 = "486",
                0x2 = "Pentium",
                0x3 = "Pentium Pro",
                0x10 = "Alpha",
                0x20 = "PowerPC",
                0x30 = "MIPS",
                0x40 = "Co-Processor",
                0x80 = "Other",
            },
            /* Serial Bus Controller */
            0xC {
                0x0 = "FireWire (IEEE 1394) Controller",
                0x1 = "ACCESS Bus Controller",
                0x2 = "SSA",
                0x3 = "USB Controller",
                0x4 = "Fibre Channel",
                0x5 = "SMBus Controller",
                0x6 = "InfiniBand Controller",
                0x7 = "IPMI Interface",
                0x8 = "SERCOS Interface (IEC 61491)",
                0x9 = "CANbus Controller",
                0x80 = "Other",
            },
            /* Wireless Controller */
            0xD {
                0x0 = "iRDA Compatible Controller",
                0x1 = "Consumer IR Controller",
                0x10 = "RF Controller",
                0x11 = "Bluetooth Controller",
                0x12 = "Broadband Controller",
                0x20 = "Ethernet Controller (802.1a)",
                0x21 = "Ethernet Controller (802.1b)",
                0x80 = "Other",
            },
            /* Intelligent Controller */
            0xE {
                0x0 = "I20",
            },
            /* Sattelite Communication Controller */
            0xF {
                0x1 = "Satellite TV Controller",
                0x2 = "Satellite Audio Controller",
                0x3 = "Satellite Voice Controller",
                0x4 = "Satellite Data Controller",
            },
            /* Encryption Controller */
            0x10 {
                0x0 = "Network and Computing Encryption/Decryption",
                0x10 = "Entertainment Encryption/Decryption",
                0x80 = "Other",
            },
            /* Signal Processing Controller */
            0x11 {
                0x0 = "DPIO Modules",
                0x1 = "Performance Counters",
                0x10 = "Communicaion Synchronizer",
                0x20 = "Signal Processing Management",
                0x80 = "Other",
            },
            /* The following fields were emitted due to empty subclasses:
                0x12 - Processing Accelerator
                0x13 - Non-Esential Instrumentation
            */
        }
    }
}
