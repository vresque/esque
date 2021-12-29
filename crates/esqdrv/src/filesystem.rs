pub struct FileSystemObject {}

pub trait FileSystem {
    fn open() -> File;
    fn close(f: File);
    fn info(f: File);
    fn read(f: File) -> *mut () {
        (f.read)(&f as *const File)
    }
    fn write(mut f: File, bytes: *mut ()) -> u64 {
        (f.write)(&mut f as *mut File, bytes)
    }

    fn to_object() -> FileSystemObject;
}

pub struct File {
    pub read: extern "C" fn(f: *const File) -> *mut (),
    pub write: extern "C" fn(f: *mut File, content: *mut ()) -> u64,
}