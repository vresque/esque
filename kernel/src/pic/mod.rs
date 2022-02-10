//! This entire file is largely based on the examples in https://wiki.osdev.org/PIC#Programming_the_PIC_chips

use crate::iobus::{inb, io_wait, outb};

// Choose at free will
const PIC1_OFFSET: u8 = 0x20;
const PIC2_OFFSET: u8 = 0x08;

enumtastic::const_enum! {
    // https://wiki.osdev.org/PIC#Programming_the_PIC_chips
    pub enum PicPort: u16 => {
        Pic1Command = 0x20,
        Pic1Data = 0x21,
        Pic2Command = 0xA0,
        Pic2Data = 0xA1,
        // Utility
        Ps2KeyboardScancodePort = 0x60,
        PitPort = 0x40,
        PicMouseScancodePort = 0x60,
    }

    impl {}
}

enumtastic::const_enum! {
    pub enum PicUtilValue: u8 => {
        Pic1Mask = 0b11111000,
        Pic2Mask = 0b11101111,
    }

    impl {}
}

enumtastic::const_enum! {
    pub enum PicInterrupt: u8 => {
        Ps2KeyboardInterrupt = PIC1_OFFSET + 0x1,
        Ps2MouseInterrupt = 0x2C,
    }

    impl {}
}

enumtastic::const_enum! {
    // https://wiki.osdev.org/PIC#Programming_the_PIC_chips
    pub enum PicValue: u8 => {
        Icw1Init = 0x10, /* Initialization - Required!! */
        Icw1Level = 0x08, /* Level triggered (edge) mode */
        Icw1Interval4 = 0x04, /* Call address interval 4 (8) */
        Icw1Single = 0x02, /* Single (cascade mode) */
        Icw1_Icw4 = 0x01, /* ICW4 (not) needed */

        Icw4_8086 = 0x01, /* 8086/88 (MCS-80/75) mode */
        Icw4AutoEndOfInput = 0x02, /* Auto (normal) EndOfInput */
        Icw4BufMinor = 0x08, /* Buffered mode/minor */
        Icw4BufMain = 0x0C, /* Buffered mode/main */
        Icw4Sfnm = 0x10, /* Special fully nested */
        EndOfInterrupt = 0x20,
    }

    impl {}
}

/// # End Main Pic
/// End the input of the main pic
pub fn end_main_pic() {
    outb(PicPort::Pic1Command, PicValue::EndOfInterrupt);
}

/// # End Minor Pic
/// End the input of the minor pic
pub fn end_minor_pic() {
    // Both Chips must be ended
    outb(PicPort::Pic2Command, PicValue::EndOfInterrupt);
    outb(PicPort::Pic1Command, PicValue::EndOfInterrupt);
}

/// # Remap PIC
/// Remaps the PIC
/// Note: The io_wait is necessary to give older machines time to react
/// Translated into rust from https://wiki.osdev.org/PIC#Programming_the_PIC_chips
pub fn remap_pic(offset_1: u8, offset_2: u8) {
    // Store the masks of the chips
    let chip1: u8 = inb(PicPort::Pic1Data);
    io_wait();
    let chip2: u8 = inb(PicPort::Pic2Data);
    io_wait();

    // Initializing the PICs
    outb(
        PicPort::Pic1Command,
        PicValue::Icw1Init | PicValue::Icw1_Icw4,
    ); // Initialize Chip 1 in cascade mode
    io_wait();
    outb(
        PicPort::Pic2Command,
        PicValue::Icw1Init | PicValue::Icw1_Icw4,
    ); // Initialize Chip 2 in cascade mode
    io_wait();

    // Setting offsets
    outb(PicPort::Pic1Data, offset_1); // ICW2: Set IDT vector offset for Main Pic
    io_wait();
    outb(PicPort::Pic2Data, offset_2); // ICW2: Set IDT vector offset for Minor Pic
    io_wait();

    // Inform the PICs of themselves / the other pic
    outb(PicPort::Pic1Data, 4); // ICW3: Tell main pic that there is a minor pic at IRQ2 (0000 0100)
    io_wait();
    outb(PicPort::Pic2Data, 2); // ICW3: Tell minor pic its cascade identity (0000 0010)

    // Operate in Icw4 8086 mode
    outb(PicPort::Pic1Data, PicValue::Icw4_8086);
    io_wait();
    outb(PicPort::Pic2Data, PicValue::Icw4_8086);
    io_wait();

    // Restore the saved masks
    outb(PicPort::Pic1Data, chip1);
    io_wait();
    outb(PicPort::Pic2Data, chip2);
    io_wait();
}
