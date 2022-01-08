#![no_std]

use core::ptr::NonNull;

/// # Unique
/// A "thread-safe" wrapper around NonNull with the additional bonus of
/// being able to supply const pointers
#[derive(Debug, Eq, PartialEq)]
pub struct Unique<T>(NonNull<T>);

unsafe impl<T> Send for Unique<T> {}
unsafe impl<T> Sync for Unique<T> {}
impl<T> Copy for Unique<T> {}
impl<T> Clone for Unique<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Unique<T> {
    pub fn new_const(ptr: *const T) -> Option<Self> {
        Self::new(ptr as *mut T)
    }

    pub unsafe fn new_const_unchecked(ptr: *const T) -> Self {
        Self::new_unchecked(ptr as *mut T)
    }

    pub fn new(ptr: *mut T) -> Option<Self> {
        let nonnull = match NonNull::new(ptr) {
            None => return None,
            Some(nn) => nn,
        };
        Some(Self(nonnull))
    }

    pub unsafe fn new_unchecked(ptr: *mut T) -> Self {
        Self(NonNull::new_unchecked(ptr))
    }

    pub fn new_borrowed_mut(x: &mut T) -> Option<Self> {
        Self::new(x as *mut T)
    }

    pub fn new_borrowed(x: &T) -> Option<Self> {
        Self::new_const(x as *const T)
    }

    pub fn inner<'retval>(self) -> NonNull<T> {
        self.0
    }

    pub fn as_ptr(self) -> *const T {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_ptr()
    }

    pub unsafe fn as_mut(&mut self) -> &mut T {
        self.0.as_mut()
    }
}
