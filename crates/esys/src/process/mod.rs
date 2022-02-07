pub mod pid;
use pid::Pid;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Process {
    pid: Pid,
    entry: u64,
    ipc_base: u64,
    protected: bool,
}

impl Process {
    pub fn new(pid: Pid, entry: u64, ipc_base: u64, protected: bool) -> Self {
        Self {
            pid,
            entry,
            ipc_base,
            protected,
        }
    }
}
