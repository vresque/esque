use bks::Handover;

use crate::{
    iobus::outb,
    pic::{self, PicPort, PicUtilValue},
};

pub fn init_pic(_: &mut Handover) {
    pic::remap_pic(0x20, 0x08);

    // Unmask the PS2-Keyboard Interrupts
    outb(PicPort::Pic1Data, PicUtilValue::Pic1UnmaskPs2Keyboard);
    outb(PicPort::Pic2Data, PicUtilValue::Pic2MaskFully);

    // Reload Interrupt Flags
    unsafe {
        core::arch::asm!("sti");
    };
}
