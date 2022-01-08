use bks::Handover;

use crate::scheduler::pit::*;

pub fn init_pit(_: &mut Handover) {
    set_divisor(DIVISOR_MAX / 10);
}
