pub mod bitmap;
pub mod memset;
pub mod paging;
pub use memset::memset;
pub mod allocator;
pub mod userspace;

#[macro_export]
macro_rules! address_of {
    ($x:ident) => {
        $x as *const _ as *const u64 as u64
    };
}
