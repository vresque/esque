#[repr(C)]
#[derive(Copy, Clone)]
pub struct FileSystemObject {
    pub buf1: u64, // The FileSystem is given three buffers to store information in
    pub buf2: u64,
    pub buf3: u64,
    pub open_fs: fn(),
    pub close_fs: fn(me: *mut FileSystemObject),
    pub open: fn(me: *mut FileSystemObject, path: *mut u8) -> *mut File,
    pub close: fn(me: *mut FileSystemObject, fd: *mut File),
    pub info: fn(me: *mut FileSystemObject, fd: *mut File) -> *mut FileInfo,
    pub read: fn(me: *mut FileSystemObject, fd: *mut File, buf: *mut u8, size: u64),
    pub write: fn(me: *mut FileSystemObject, fd: *mut File, buf: *mut u8, size: u64),
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FileInfo {
    pub size: u64,
}

pub unsafe trait FileSystem {
    fn open_fs();
    fn close_fs(me: *mut FileSystemObject);
    fn open(me: *mut FileSystemObject, path: *mut u8) -> *mut File;
    fn close(me: *mut FileSystemObject, fd: *mut File);
    fn info(me: *mut FileSystemObject, fd: *mut File) -> *mut FileInfo;
    fn read(me: *mut FileSystemObject, fd: *mut File, buf: *mut u8, size: u64);
    fn write(me: *mut FileSystemObject, fd: *mut File, buf: *mut u8, size: u64);
    fn to_object(buf1: u64, buf2: u64, buf3: u64) -> FileSystemObject {
        FileSystemObject {
            buf1,
            buf2,
            buf3,
            open_fs: Self::open_fs,
            close_fs: Self::close_fs,
            open: Self::open,
            close: Self::close,
            info: Self::info,
            read: Self::read,
            write: Self::write,
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct File {
    pub read: extern "C" fn(f: *const File, buf: *mut u8, size: u64),
    pub write: extern "C" fn(f: *mut File, content: *mut u8, size: u64) -> u64,
    pub buf: u64,
    pub info: FileInfo,
}
