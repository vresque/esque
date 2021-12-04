use bks::Handover;
use crate::{log::{FramebufferGuard, Color, FRAMEBUFFER_GUARD}, kprintln};

pub fn init_common(mut handover: &mut Handover) {
    unsafe { FRAMEBUFFER_GUARD.write(FramebufferGuard::new(*handover.framebuffer(), *handover.font(), Color::Black, Color::White)); }
    kprintln!("Hello, World (Lol)");
}