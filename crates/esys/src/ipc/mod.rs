// The following structure must be at the beginning of each IPC Queue
pub struct IPCQueueHeader {
    pub checknum: u8, // 42
    pub queue: [IPCMessage; 255],
    pub slots: [u8; 255], // TRUE if the slot is occupied, FALSE if not
    pub is_empty: u8,
}

#[repr(align(0x80))]
pub struct IPCMessage {
    // The following must be 42 - This is to check that the structure is, in fact, an ipc message
    pub checknum: u8,
    // In the case of future IPC versions
    pub version: u8,
    // If the IPC was answered and handled by the kernel
    pub is_answered: u8,
    // The kernel error code; 0 = SUCCESS
    pub errno: u8,
    // If the message was resolved (This must be selected by the user, the kernel will then remove the message from the stack and set the corresponding slot to false)
    pub is_resolved: u8,
    // The Lane of the Message
    // Examples:
    // 0 => KERNEL SUBSYSTEM
    // 1 => POSIX SUBSYSTEM
    pub lane: u8,
    // The Group of the Message: Example
    // 0 => KERNEL
    // 1 => SECURITY
    // 2 => FS
    // ...
    pub group: u64,
    // Some system space applications further group their messages
    // Example in the kernel
    // 0 => GetInfo
    // 1 => FileSystem
    // ...
    pub subgroup: u8,
    // The type of the message
    pub ty: u64,
    // Pointers - Any of the following may simply be 0, in which case it is NULL
    pub ptr: u64,
    pub ptr2: u64,
    pub ptr3: u64,
    pub ptr4: u64,
    // Extra Args - Other non-pointer arguments are stored here
    pub extra_arg1: u64,
    pub extra_arg2: u64,
    pub extra_arg3: u64,
    pub extra_arg4: u64,
}
