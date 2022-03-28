use core::mem::MaybeUninit;

use alloc::sync::Arc;
use bks::{Config, Handover};
use spin::{Mutex, Once, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub static KCONFIG: Mutex<Config> = Mutex::new(Config::default());

pub fn config() -> Config {
    *KCONFIG.lock()
}

pub static HANDOVER: Once<Arc<RwLock<Handover>>> = Once::new();

pub fn set_handover(mut hand: Handover) {
    HANDOVER.call_once(|| Arc::new((RwLock::new(hand))));
}

pub fn handover() -> RwLockReadGuard<'static, Handover> {
    HANDOVER.call_once(|| panic!()).read()
}

pub fn handover_mut() -> RwLockWriteGuard<'static, Handover> {
    HANDOVER.call_once(|| panic!("")).write()
}
