use alloc::vec;
use alloc::vec::Vec;
use esys::ipc::{IPCMessage, IPCQueueHeader};
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
    if header.as_mut().checknum != 42 || header.as_mut().is_empty == true as u8 {
        return; // Corrupt Queue TODO: Remove Queue
    }

    for slot in header.as_mut().slots {
        if slot == true as u8 {
            let ipc = &mut header.as_mut().queue[slot as usize];

            if ipc.lane != 0 {
                return; /* Lane 0 is the kernel lane */
            }

            // Everything other than 0 needs to be handled by another department
            match ipc.group {
                0 => kernel_handle_ipc_message(ipc),
                1 => {}
                _ => return,
            }

            // TODO: Handle IPC Message

            ipc.is_answered = true as u8;
            header.as_mut().slots[slot as usize] = true as u8;
        }
    }
}

pub fn kernel_handle_ipc_message(msg: &mut IPCMessage) {}

// Dispatches a message to another system space application
pub fn dispatch_message_internal(msg: &mut IPCMessage, receiver: u32) {}
