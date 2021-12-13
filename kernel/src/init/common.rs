use crate::{
    framebuffer::{Color, FramebufferGuard, FRAMEBUFFER_GUARD},
    kprintln,
    memory::paging::page_table_manager::PageTable,
};
use bks::Handover;

pub fn init_common(handover: &mut Handover) {
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

        kprintln!("Initialized Logging!");

        for i in 0..1000 {
            kprintln!("{i}");
        }
    };
}
