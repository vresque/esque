pub use crate::error::Result;
pub use esyscall_support::stat::Stat;

pub trait Device {
    #[allow(unused)]
    fn open(&self, path: &str, flags: u64, uid: u32, gid: u32) -> Result<u32>;
    #[allow(unused)]
    fn read(&self, id: usize, buffer: &mut [u8]) -> Result<u32> {
        Ok(0)
    }
    #[allow(unused)]
    fn seek(&self, id: usize, position: isize, whence: usize) -> Result<isize> {
        Ok(0)
    }
    #[allow(unused)]
    fn fcntl(&self, id: usize, command: usize, arg: usize) -> Result<u32> {
        Ok(0)
    }
    #[allow(unused)]

    fn fpath(&self, id: usize, buf: &mut [u8]) -> Result<u32> {
        Ok(0)
    }
    #[allow(unused)]

    fn fstat(&self, id: usize, stat: &mut Stat) -> Result<u32> {
        Ok(0)
    }
    #[allow(unused)]

    fn fsync(&self, id: usize) -> Result<u32> {
        Ok(0)
    }
    #[allow(unused)]

    fn close(&self, id: usize) -> Result<u32> {
        Ok(0)
    }
    #[allow(unused)]

    fn chmod(&self, path: &str, mode: u32, gid: u32) -> Result<u32> {
        Ok(0)
    }
    #[allow(unused)]

    fn rmdir(&self, path: &str, uid: u32, gid: u32) -> Result<u32> {
        Ok(0)
    }
}
