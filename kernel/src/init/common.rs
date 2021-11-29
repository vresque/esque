use bks::Handover;
use crate::{log::{FramebufferWriter, FRAMEBUFFER_WRITER, Color}, HANDOVER};

pub fn init_common() {
    unsafe { FRAMEBUFFER_WRITER = Some(FramebufferWriter::new(&mut HANDOVER.unwrap());