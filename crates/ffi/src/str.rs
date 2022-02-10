use alloc::string::String;

extern crate alloc;

/// # CString
/// Representing a fixed-size CString
/// ## Note
/// This can be used instead of [u8; N]
pub struct CStr<const N: usize>([u8; N]);

impl<const N: usize> CStr<N> {
    pub fn as_str(&self) -> String {
        String::from("Helo!")
    }
}
