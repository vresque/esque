pub use crate::error::Result;
pub use esyscall_support::stat::Stat;

pub trait Stratagem {
    fn open(&self, path: &str, flags: u64, uid: u32, gid: u32) -> Result<usize>;
    fn read(&self, id: usize, buffer: &mut [u8]) -> Result<usize>;
    fn seek(&self, id: usize, position: isize, whence: usize) -> Result<isize>;
    fn fcntl(&self, id: usize, command: usize, arg: usize) -> Result<usize>;
    fn fpath(&self, id: usize, buf: &mut [u8]) -> Result<usize>;
    fn fstat(&self, id: usize, stat: &mut Stat) -> Result<usize>;
    fn fsync(&self, id: usize) -> Result<usize>;
    fn close(&self, id: usize) -> Result<usize>;
}
