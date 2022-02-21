pub use crate::error::Result;
pub use esyscall_support::stat::Stat;

pub trait Device {
    fn open(&self, path: &str, flags: u64, uid: u32, gid: u32) -> Result<u32>;
    fn read(&self, id: usize, buffer: &mut [u8]) -> Result<u32> {
        Ok(0)
    }
    fn seek(&self, id: usize, position: isize, whence: usize) -> Result<isize> {
        Ok(0)
    }
    fn fcntl(&self, id: usize, command: usize, arg: usize) -> Result<u32> {
        Ok(0)
    }
    fn fpath(&self, id: usize, buf: &mut [u8]) -> Result<u32> {
        Ok(0)
    }
    fn fstat(&self, id: usize, stat: &mut Stat) -> Result<u32> {
        Ok(0)
    }
    fn fsync(&self, id: usize) -> Result<u32> {
        Ok(0)
    }
    fn close(&self, id: usize) -> Result<u32> {
        Ok(0)
    }
    fn chmod(&self, path: &str, mode: u32, gid: u32) -> Result<u32> {
        Ok(0)
    }
    fn rmdir(&self, path: &str, uid: u32, gid: u32) -> Result<u32> {
        Ok(0)
    }
}
