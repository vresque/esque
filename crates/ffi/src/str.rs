use core::ops::Add;

use alloc::string::{String, ToString};

extern crate alloc;

/// # Fixed CString
/// Representing a fixed-size CString
/// ## Note
/// This can be used instead of [u8; N]
pub struct FixedCStr<const N: usize>([u8; N]);

impl<const N: usize> FixedCStr<N> {
    pub fn to_string(&self) -> String {
        return String::from_utf8_lossy(&self.0).to_string();
    }

    pub fn len(&self) -> usize {
        let mut len: usize = 0;
        for c in self.0 {
            if c == '\0' as u8 {
                len += 1;
            }
        }
        len
    }

    pub fn new(arr: [u8; N]) -> Self {
        Self(arr)
    }
}

pub struct DynamicCStr<'lt>(&'lt [u8]);

impl<'lt> DynamicCStr<'lt> {
    pub fn new(arr: &'lt [u8]) -> Self {
        Self(arr)
    }

    pub fn from_ptr(ptr: *const u8) -> Self {
        Self::from_ptr_given_len(ptr, c_str_ptr_len(ptr))
    }

    pub fn from_ptr_given_len(ptr: *const u8, len: usize) -> Self {
        unsafe { Self::new(core::slice::from_raw_parts(ptr, len)) }
    }

    fn len(&self) -> usize {
        let mut len: usize = 0;
        for c in self.0 {
            if *c == '\0' as u8 {
                len += 1;
            }
        }
        len
    }

    pub fn to_string(&self) -> String {
        return String::from_utf8_lossy(self.0).to_string();
    }
}

pub trait FFIStringExt {
    fn from_c_dyn_slice(data: &[u8]) -> Self;
    unsafe fn from_c_ptr(ptr: *const u8) -> Self;
    fn from_c_fixed_slice<const N: usize>(data: [u8; N]) -> Self;
    unsafe fn from_c_ptr_size_given(ptr: *const u8, len: usize) -> Self;
}

impl FFIStringExt for String {
    fn from_c_dyn_slice(data: &[u8]) -> Self {
        DynamicCStr::new(data).to_string()
    }

    unsafe fn from_c_ptr(ptr: *const u8) -> Self {
        Self::from_c_ptr_size_given(ptr, c_str_ptr_len(ptr))
    }

    fn from_c_fixed_slice<const N: usize>(data: [u8; N]) -> Self {
        FixedCStr::<N>::new(data).to_string()
    }

    unsafe fn from_c_ptr_size_given(ptr: *const u8, len: usize) -> Self {
        DynamicCStr::new(core::slice::from_raw_parts(ptr, len)).to_string()
    }
}

pub fn c_str_ptr_len(ptr: *const u8) -> usize {
    let mut len = 0usize;
    while unsafe { ptr.add(1) } != core::ptr::null() {
        len += 1;
    }
    len
}
