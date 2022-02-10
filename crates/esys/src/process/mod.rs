pub mod pid;
use pid::Pid;

pub trait Process {
    fn kill();
    fn signal(signal: u64);
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SystemProcess {
    pub pid: Pid,
    pub entry: u64,
    pub msg_base: u64,
    pub protected: bool,
}

impl SystemProcess {
    pub fn new(pid: Pid, entry: u64, msg_base: u64, protected: bool) -> Self {
        Self {
            pid,
            entry,
            msg_base,
            protected,
        }
    }
}
