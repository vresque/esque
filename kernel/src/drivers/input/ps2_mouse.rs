use enumtastic::const_enum;

use crate::{interrupts::interrupt_frame::InterruptFrame, debug, pic, iobus::{outb, inb}};

pub const MOUSE_TIMEOUT: u64 = 100_000;

const_enum! {
    pub enum MouseButton: u64 => {
        LeftClick = 0,
        RightClick = 0,
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
    }

    impl {}
}

pub extern "x86-interrupt" fn ps2_mouse_interrupt_handler(_a: InterruptFrame) {
    debug!("Mouse handler called");


    pic::end_minor_pic();
}


pub fn mouse_wait_for_input_reading() {
        // What this does is the following:
    // It waits for a maximum of MOUSE_TIMEOUT
    // unless the bit on the device port is cleared
    // This waits for the input instead of for a clear
    for i in 0..MOUSE_TIMEOUT {
        if (inb(Ps2MousePicPort::CommandPort) & 0) != 0 {
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
    for i in 0..MOUSE_TIMEOUT {
        if (inb(Ps2MousePicPort::CommandPort) & 1) != 0 {
            return;
        }
    }
}

// Enables the mouse
pub fn ps2_mouse_init() {
    // Enable the Auxiliary Device "mouse"
    // osdev.org :: https://wiki.osdev.org/Mouse_Input
    // >  Waiting to Send Bytes to Port 0x60 and 0x64
    // > All output to port 0x60 or 0x64 must be preceded by waiting for bit 1 (value=2) of port 0x64 to become clear. Similarly, bytes cannot be read from port 0x60 until bit 0 (value=1) of port 0x64 is set. See PS2 Keyboard for further details. 
    outb(Ps2MousePicPort::CommandPort, Ps2MousePicValue::EnableAuxiliaryDeviceMouse);

    // Always wait before sending the next
    mouse_wait();
    outb(Ps2MousePicPort::CommandPort, Ps2MousePicValue::PrepareForCommand);

    let status = inb(Ps2MousePicPort::DataPort);
}