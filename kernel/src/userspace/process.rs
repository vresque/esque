use super::pid::Pid;

#[repr(C)]
pub struct Process {
    pid: Pid,
    entry: u64,
    msgbuf_base: u64,
}
