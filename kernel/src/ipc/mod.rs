pub static IPC_QUEUES: Mutex<Vec<u64>> = vec![];


pub fn kernel_ipc_handler() {
    for queue in IPC_QUEUES.lock() {
        handle_ipc_queue(queue);
    }
}

pub fn handle_ipc_queue(queue: u64) {}