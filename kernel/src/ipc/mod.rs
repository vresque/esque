use alloc::vec;
use alloc::vec::Vec;
use esys::ipc::IPCQueueHeader;
use spin::Mutex;
use unique::Unique;

pub static IPC_QUEUES: Mutex<Vec<u64>> = Mutex::new(vec![]);

pub fn kernel_ipc_handler() {
    for queue in &*IPC_QUEUES.lock() {
        handle_ipc_queue(*queue);
    }
}

pub fn handle_ipc_queue(queue: u64) {
    let mut header = Unique::new(queue as *mut u64 as *mut IPCQueueHeader).unwrap();
    if header.as_mut().checknum != 42 {
        return; // Corrupt Queue TODO: Remove Queue
    }
}
