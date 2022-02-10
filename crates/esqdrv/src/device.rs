#[repr(C)]
pub struct FileOperations {
    //open: extern "C" fn(node: *mut FsNode, file: *mut File) -> i32,
//close: extern "C" fn(node: *mut FsNode, file: *mut File) -> i32,
//read: extern "C" fn(file: *mut File, buf: *mut u8, len: usize, offset: *mut isize) -> isize,
//write: extern "C" fn(file: *mut File, buf: *const u8, len: usize, offset: *mut isize) -> isize,
//commands: [*mut (); 251], // 251 (COMMAND_MAX - 4 (255 minus open, close, read and write)) of void*ers representing functions commands[0] is equal to calling actual_commands[0 + 4]
}

#[repr(C)]
pub struct Device {
    name: [u8; 255], // C-Compatibility
    operations: FileOperations,
}
