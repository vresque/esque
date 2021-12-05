use crate::{
    kprintln,
    log::{Color, FramebufferGuard, FRAMEBUFFER_GUARD},
};
use bks::Handover;

pub fn init_common(mut handover: &mut Handover) {
    unsafe {
        FRAMEBUFFER_GUARD.lock().write(FramebufferGuard::new(
            *handover.framebuffer(),
            *handover.font(),
            Color::Black,
            Color::White,
        ));

        FRAMEBUFFER_GUARD
            .lock()
            .assume_init_mut()
            .clear_color(Color::Black);
    }
    kprintln!("Initialized Logging!");
}
