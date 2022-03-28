pub mod bitmap;
pub mod memset;
pub mod paging;
pub mod structures;
pub use memset::memset;
pub mod allocator;
pub mod userspace;
pub use structures::*;

#[macro_export]
macro_rules! address_of {
    ($x:expr) => {
        ($x) as *const _ as *const u64 as u64
    };
}

#[macro_export]
/// # From Address
/// Gets a reference from an address
/// ## Possible Invokations
/// ```
/// // The following two expressions are equal
/// let x: &MyType = from_addr!(0xFF23f34)
/// let y = from_addr!(MyType: 0xFF23f34)
/// // So are the next two
/// let mut_x: &mut MyType = from_addr!(mut 0xFF23f34)
/// let mut_y = from_addr!(mut MyType: 0xFF23f34)
/// ```
macro_rules! from_addr {
    ($a:expr) => {
        (unsafe { &*(($a) as *const u64 as *const _) })
    };
    ($ty:ty: $a:expr) => {
        (unsafe { &*(($a) as *const u64 as *const $ty) })
    };
    (mut $ty:ty: $a:expr) => {
        (unsafe { &mut *(($a) as *mut u64 as *mut $ty) })
    };
    (mut $a:expr) => {
        (unsafe { &mut *(($a) as *mut u64 as *mut _) })
    };
}
