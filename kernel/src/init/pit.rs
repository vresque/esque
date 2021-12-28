use bks::Handover;

use crate::{debug, scheduler::pit::*};

pub fn init_pit(_: &mut Handover) {
    set_divisor(DIVISOR_MAX / 10);
    for i in 0..20 {
        debug!("Shhhhhh....");
        sleep(2 as f64);
    }
}
