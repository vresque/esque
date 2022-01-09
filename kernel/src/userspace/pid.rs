use core::sync::atomic::{AtomicUsize, Ordering};

use crate::{debug, scheduler::pit::TIME_SINCE_BOOT};

pub static LAST_PID: AtomicUsize = AtomicUsize::new(0);
pub use esys::process::pid::Pid;

pub trait KernelPid {
    fn new(pid: usize) -> Self;
    fn force_new(pid: usize) -> Self;
    fn random() -> Self;
}

impl KernelPid for Pid {
    fn new(pid: usize) -> Self {
        if pid <= LAST_PID.load(core::sync::atomic::Ordering::Acquire) {
            return Self::new(LAST_PID.load(Ordering::Acquire) + 1);
        }
        LAST_PID.store(pid, Ordering::SeqCst);
        Self { id: pid }
    }

    fn force_new(pid: usize) -> Self {
        LAST_PID.store(pid, Ordering::SeqCst);
        Self { id: pid }
    }

    fn random() -> Self {
        let last_pid = LAST_PID.load(Ordering::SeqCst);
        // Pseudo-Random PID
        let time = (TIME_SINCE_BOOT.lock().read() * 100000000.0) as usize;
        debug!("T: {}, L: {}", time, last_pid);
        let new_pid_full =
            (((((time / 3) as usize | last_pid) & !last_pid) % (time / 2) as usize) << 4) | 3;
        debug!("{}", new_pid_full);
        let lo = new_pid_full & 0x00ff;
        let hi = new_pid_full & 0xff00;
        let new = hi | lo;
        let nearly_there = new & time;
        let pid = ((nearly_there | hi) & lo) >> 5;
        Self::new(pid)
    }
}
