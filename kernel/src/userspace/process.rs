use super::pid::Pid;

#[repr(C)]
pub struct Process {
    pid: Pid,
    entry: u64,
    msgbuf_base: u64,
    protected: bool,
}

impl Process {
    pub fn new(pid: Pid, entry: u64, msgbuf_base: u64, protected: bool) -> Self {
        Self {
            pid,
            entry,
            msgbuf_base,
            protected,
        }
    }
}
