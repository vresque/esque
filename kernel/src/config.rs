use bks::Config;
use spin::Mutex;

pub static KCONFIG: Mutex<Config> = Mutex::new(Config::default());

pub fn config() -> Config {
    *KCONFIG.lock()
}
