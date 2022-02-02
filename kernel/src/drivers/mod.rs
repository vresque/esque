use bks::Handover;

pub mod input;

pub fn init_drivers(_: &mut Handover) {
    input::ps2_mouse::ps2_mouse_init();
}
