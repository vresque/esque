use bks::Handover;

use crate::{
    arch::pic::{self, PicPort, PicUtilValue},
    debug,
    iobus::outb,
    kprintln,
};

pub fn init_pic(_: &mut Handover) {
    crate::info!("Initializing the PIC");
    pic::remap_pic(0x20, 0x08);

    // Unmask the PS2-Keyboard Interrupts
    outb(PicPort::Pic1Data, PicUtilValue::Pic1Mask);
    outb(PicPort::Pic2Data, PicUtilValue::Pic2Mask);

    // Reload Interrupt Flags
    unsafe {
        comasm::reload_interrupt_flags();
    };
    debug!("Initialized PIC");
}
