use bks::Handover;

use crate::config::KCONFIG;

pub fn init_config(handover: &mut Handover) {
    *KCONFIG.lock() = handover.config;
}
