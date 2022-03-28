use enumtastic::const_enum;

use crate::{
    arch::interrupts::interrupt_frame::InterruptFrame,
    arch::iobus::{inb, outb},
    arch::pic,
    debug,
};

pub const MOUSE_TIMEOUT: u64 = 100_000;

const_enum! {
    pub enum MouseButton: u64 => {
        LeftClick = 0,
        RightClick = 0,
    }

    impl {}
}

const_enum! {
    pub enum Ps2MouseWriteValues: u8 => {
        UseDefaultSettings = 0xF6,
        EnableDataReporting = 0xF4,
    }

    impl {}
}

const_enum! {
    pub enum Ps2MousePicPort: u16 => {
        // osdev.org :: https://wiki.osdev.org/Mouse_Input
        // >  Waiting to Send Bytes to Port 0x60 and 0x64
        // > All output to port 0x60 or 0x64 must be preceded by waiting for bit 1 (value=2) of port 0x64 to become clear. Similarly, bytes cannot be read from port 0x60 until bit 0 (value=1) of port 0x64 is set. See PS2 Keyboard for further details.
        CommandPort = 0x64,
        DataPort = 0x60,
    }

    impl {}
}

const_enum! {
    pub enum Ps2MousePicValue: u8 => {
        // osdev.org :: https://wiki.osdev.org/Mouse_Input
        // >  Waiting to Send Bytes to Port 0x60 and 0x64
        // > All output to port 0x60 or 0x64 must be preceded by waiting for bit 1 (value=2) of port 0x64 to become clear. Similarly, bytes cannot be read from port 0x60 until bit 0 (value=1) of port 0x64 is set. See PS2 Keyboard for further details.
        EnableAuxiliaryDeviceMouse = 0xA8,
        PrepareForCommand = 0x20,
        PrepareForData = 0x60,
        AddressTheMouse = 0xD4,
        WriteMouseCommandToData = 0xF3,
    }

    impl {}
}

pub extern "x86-interrupt" fn ps2_mouse_interrupt_handler(_a: InterruptFrame) {
    // Read the input
    let _data = inb(Ps2MousePicPort::DataPort);
    debug!("Mouse handler called");

    pic::end_minor_pic();
}

pub fn mouse_read() -> u8 {
    mouse_wait_for_input_reading();
    return inb(Ps2MousePicPort::DataPort);
}

pub fn mouse_wait_for_input_reading() {
    // What this does is the following:
    // It waits for a maximum of MOUSE_TIMEOUT
    // unless the bit on the device port is cleared
    // This waits for the input instead of for a clear
    for _ in 0..MOUSE_TIMEOUT {
        if (inb(Ps2MousePicPort::CommandPort) & 0b1) != 0 {
            return;
        }
    }
}

pub fn mouse_wait() {
    // osdev.org :: https://wiki.osdev.org/Mouse_Input
    // >  Waiting to Send Bytes to Port 0x60 and 0x64
    // > All output to port 0x60 or 0x64 must be preceded by waiting for bit 1 (value=2) of port 0x64 to become clear. Similarly, bytes cannot be read from port 0x60 until bit 0 (value=1) of port 0x64 is set. See PS2 Keyboard for further details.

    // What this does is the following:
    // It waits for a maximum of MOUSE_TIMEOUT
    // unless the bit on the device port is cleared
    for _ in 0..MOUSE_TIMEOUT {
        if (inb(Ps2MousePicPort::CommandPort) & 0b10) == 0 {
            return;
        }
    }
}

pub fn mouse_write(value: u8) {
    // We must address the mouse (not the keyboard)
    outb(
        Ps2MousePicPort::CommandPort,
        Ps2MousePicValue::AddressTheMouse,
    );
    mouse_wait();
    outb(Ps2MousePicPort::DataPort, value);
}

// Enables the mouse
pub fn ps2_mouse_init() {
    debug!("Initializing PS2 Mouse");
    // Enable the Auxiliary Device "mouse"
    // osdev.org :: https://wiki.osdev.org/Mouse_Input
    // >  Waiting to Send Bytes to Port 0x60 and 0x64
    // > All output to port 0x60 or 0x64 must be preceded by waiting for bit 1 (value=2) of port 0x64 to become clear. Similarly, bytes cannot be read from port 0x60 until bit 0 (value=1) of port 0x64 is set. See PS2 Keyboard for further details.
    // ...
    // > Aux Input Enable Command
    // > Send the Enable Auxiliary Device command (0xA8) to port 0x64. This will generate an ACK response from the keyboard, which you must wait to receive. Please note that using this command is not necessary if setting the Compaq Status byte is successful -- but it does no harm, either.
    outb(
        Ps2MousePicPort::CommandPort,
        Ps2MousePicValue::EnableAuxiliaryDeviceMouse,
    );

    // Enable the 'compaq' data byte
    // osdev.org :: https://wiki.osdev.org/Mouse_Input
    // > After the PS2 Aux port has been enabled, you can send commands to the mouse.
    // > It is recommended to disable automatic packet streaming mode while "reprogramming" the mouse. You can do this by either sending command 0xF5 to the mouse, or disabling the "master mouse clock" by setting bit 5 of the Compaq Status byte (see below).

    // Always wait before sending the next
    mouse_wait();
    // osdev.org ::
    // > ... On some systems, the PS2 aux port is disabled at boot. Data coming from the aux port will not generate any interrupts. To know that data has arrived, you need to enable the aux port to generate IRQ12. There is only one way to do that, which involves getting/modifying the "compaq status" byte. You need to send the command byte 0x20 ("Get Compaq Status Byte") to the PS2 controller on port 0x64. ...
    // Context: PrepareForCommand = 0x20
    outb(
        Ps2MousePicPort::CommandPort,
        Ps2MousePicValue::PrepareForCommand,
    );
    mouse_wait_for_input_reading();
    let mut status = inb(Ps2MousePicPort::DataPort);
    status |= 0b10;
    mouse_wait();

    outb(
        Ps2MousePicPort::CommandPort,
        Ps2MousePicValue::PrepareForData,
    );
    mouse_wait();

    outb(Ps2MousePicPort::DataPort, status);
    mouse_wait();

    // Use the default mouse settings
    mouse_write(Ps2MouseWriteValues::UseDefaultSettings);
    mouse_wait_for_input_reading();
    mouse_read();

    mouse_write(Ps2MouseWriteValues::EnableDataReporting);
    mouse_wait_for_input_reading();
    mouse_read();
}
