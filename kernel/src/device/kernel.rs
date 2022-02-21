use alloc::{fmt::format, string::ToString};
use rlibc::memcpy;

use super::{traits::Device, DeviceID};

pub struct Kernel {
    id: DeviceID,
}

impl Kernel {
    pub fn new(id: DeviceID) -> Kernel {
        Self { id }
    }
}

impl Device for Kernel {
    fn open(&self, path: &str, flags: u64, uid: u32, gid: u32) -> super::traits::Result<u32> {
        Ok(0) /* Always open */
    }

    fn read(&self, id: usize, buffer: &mut [u8]) -> super::traits::Result<u32> {
        unsafe {
            let (ptr, size) = {
                format!(
                    "
                    {
                        \"kernel\": \"esque\",
                        \"version\": {},
                        \"authors\": {},
                    }
            ",
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_AUTHORS"),
                )
                .as_bytes();

                (buffer.as_ptr(), buffer.len())
            };

            memcpy(buffer.as_mut_ptr(), ptr, size);
            Ok(size as u32)
        }
    }

    fn seek(&self, id: usize, position: isize, whence: usize) -> super::traits::Result<isize> {
        Ok(0)
    }

    fn fcntl(&self, id: usize, command: usize, arg: usize) -> super::traits::Result<u32> {
        Ok(0)
    }

    fn fpath(&self, id: usize, buf: &mut [u8]) -> super::traits::Result<u32> {
        Ok(0)
    }

    fn fstat(
        &self,
        id: usize,
        stat: &mut esyscall_support::stat::Stat,
    ) -> super::traits::Result<u32> {
        Ok(0)
    }

    fn fsync(&self, id: usize) -> super::traits::Result<u32> {
        Ok(0)
    }

    fn close(&self, id: usize) -> super::traits::Result<u32> {
        Ok(0)
    }

    fn chmod(&self, path: &str, mode: u32, gid: u32) -> super::traits::Result<u32> {
        Ok(0)
    }

    fn rmdir(&self, path: &str, uid: u32, gid: u32) -> super::traits::Result<u32> {
        Ok(0)
    }
}
