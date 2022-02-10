use core::marker::PhantomData;

use esys::ipc::IPCQueueHeader;
use esys::process::SystemProcess;

use crate::address_of;
use crate::heap::malloc;
use crate::ipc::IPC_QUEUES;

use super::pid::KernelPid;
use super::pid::Pid;
#[derive(Copy, Clone)]
pub struct SystemSpace;
#[derive(Copy, Clone)]
pub struct Userspace;

#[derive(Debug, Copy, Clone)]
pub struct Launchpad<'data, T> {
    data: &'data [u8],
    pid: Pid,
    _phantom: PhantomData<T>,
    protected: bool,
}

impl<'data> Launchpad<'data, SystemSpace> {
    pub fn new(data: &'data [u8], protected: bool) -> Self {
        Self {
            data,
            pid: (Pid::random()),
            protected,
            _phantom: PhantomData,
        }
    }

    pub fn with_pid<'me>(mut self, pid: Pid) -> Self {
        self.pid = pid;
        return self;
    }

    pub fn launch(self) -> SystemProcess {
        let msg_base = address_of!(malloc::<IPCQueueHeader>());
        IPC_QUEUES.lock().push(msg_base);

        SystemProcess {
            pid: self.pid,
            entry: 0,
            msg_base: msg_base,
            protected: self.protected,
        }
    }
}
