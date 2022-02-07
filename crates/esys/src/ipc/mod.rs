// The following structure must be at the beginning of each IPC Queue
pub struct IPCQueueHeader {
    checknum: u8,     // 42
    slots: [u8; 255], // TRUE if the slot is occupied, FALSE if not
    is_empty: u8,
}

#[repr(align(0x80))]
pub struct IPCMessage {
    // The following must be 42 - This is to check that the structure is, in fact, an ipc message
    checknum: u8,
    // In the case of future IPC versions
    version: u8,
    // If the IPC was answered and handled by the kernel
    is_answered: u8,
    // If the message was resolved (This must be selected by the user, the kernel will then remove the message from the stack and set the corresponding slot to false)
    is_resolved: u8,
    // The Group of the Message: Example
    // 0 => KERNEL
    // 1 => SECURITY
    // 2 => FS
    // ...
    group: u64,
    // Some system space applications further group their messages
    // Example in the kernel
    // 0 => GetInfo
    // 1 => FileSystem
    // ...
    subgroup: u8,
    // The type of the message
    ty: u64,
    // Pointers - Any of the following may simply be 0, in which case it is NULL
    ptr: u64,
    ptr2: u64,
    ptr3: u64,
    ptr4: u64,
    // Extra Args - Other non-pointer arguments are stored here
    extra_arg1: u64,
    extra_arg2: u64,
    extra_arg3: u64,
    extra_arg4: u64,
}
