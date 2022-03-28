use crate::arch::scheduler::pit::*;
use bks::Handover;

pub fn init_pit(_: &mut Handover) {
    set_divisor(DIVISOR_MAX / 10);
}
