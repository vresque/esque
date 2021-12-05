use crate::{
    kprintln,
    log::{Color, FramebufferGuard, FRAMEBUFFER_GUARD},
};
use bks::Handover;

pub fn init_common(mut handover: &mut Handover) {
    unsafe {
        FRAMEBUFFER_GUARD.write(FramebufferGuard::new(
            *handover.framebuffer(),
            *handover.font(),
            Color::Cyan,
            Color::Red,
        ));
    }
    kprintln!("Hello, World (Lol)");
}
