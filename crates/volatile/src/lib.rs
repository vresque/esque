#![no_std]
#![feature(const_mut_refs)]
#![feature(const_fn_trait_bound)]

use core::marker::PhantomData;

pub struct ReadWrite;
pub struct ReadOnly;

pub struct Volatile<T, A = ReadWrite>
where
    T: Copy,
{
    value: T,
    _accessor: PhantomData<A>,
}

unsafe impl<T: Copy, A> Send for Volatile<T, A> {}
unsafe impl<T: Copy, A> Sync for Volatile<T, A> {}

impl<T: Copy> Volatile<T, ReadWrite> {
    pub const fn new_rw(val: T) -> Self {
        Self {
            value: val,
            _accessor: PhantomData,
        }
    }

    pub fn read(&self) -> T {
        // SAFETY: Reference is not null
        unsafe { core::ptr::read_volatile(&self.value as *const _) }
    }

    pub fn write(&mut self, value: T) {
        unsafe {
            core::ptr::write_volatile(&mut self.value as *mut _, value);
        }
    }

    pub fn make_readonly(&self) -> Volatile<T, ReadOnly> {
        Volatile {
            value: self.value,
            _accessor: PhantomData,
        }
    }
}

impl<T: Copy> Volatile<T, ReadOnly> {
    pub const fn new_ro(val: T) -> Self {
        Self {
            value: val,
            _accessor: PhantomData,
        }
    }

    pub fn make_writable(&self) -> Volatile<T, ReadWrite> {
        Volatile {
            value: self.value,
            _accessor: PhantomData,
        }
    }
}
