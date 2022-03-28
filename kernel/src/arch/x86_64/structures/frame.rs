use core::ops::{Add, AddAssign, Sub, SubAssign};

use super::mem::PhysicalAddress;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Frame {
    start: PhysicalAddress,
}

impl Frame {
    pub fn from_start(addr: PhysicalAddress) -> Result<Self, ()> {
        if !addr.is_aligned(bks::PAGE_SIZE) {
            return Err(());
        }
        Ok(unsafe { Self::from_start_unchecked(addr) })
    }

    pub const fn from_start_unchecked(addr: PhysicalAddress) -> Self {
        Self { start: addr }
    }

    // Returns the Frame which contains the physical address
    pub fn which_contains(addr: PhysicalAddress) -> Self {
        Self {
            start: addr.align_down_and_get(bks::PAGE_SIZE),
        }
    }

    pub const fn start(&self) -> PhysicalAddress {
        self.start
    }

    // An Iterator from start to end - exclusive end
    pub const fn range(start: Frame, end: Frame) -> PageFrameIter<false> {
        PageFrameIter { start, end }
    }

    // An Iterator from start to end - inclusive end
    pub const fn range_inclusive(start: Frame, end: Frame) -> PageFrameIter<true> {
        PageFrameIter { start, end }
    }

    // An Iterator from this frame to end - exclusive end
    pub const fn range_up_to(self, end: Frame) -> PageFrameIter<false> {
        PageFrameIter { start: self, end }
    }

    // An Iterator from this frame to end - inclusive end
    pub const fn range_up_to_inclusive(self, end: Frame) -> PageFrameIter<true> {
        PageFrameIter { start: self, end }
    }
}

impl core::fmt::Debug for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Frame")
            .field(&format_args!("{:#x?}", self.start))
            .finish()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageFrameIter<const INCLUSIVE: bool> {
    pub start: Frame,
    pub end: Frame,
}

impl core::fmt::Debug for PageFrameIter<true> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageFrameIter (Inclusive)")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl core::fmt::Debug for PageFrameIter<false> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PageFrameIter (Exclusive)")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl<const INCLUSIVE: bool> PageFrameIter<INCLUSIVE> {
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        if INCLUSIVE {
            self.start > self.end
        } else {
            self.start >= self.end
        }
    }
}

// If it is including the end page
impl Iterator for PageFrameIter<true> {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start <= self.end {
            let frame = self.start;
            self.start += 1;
            Some(frame)
        } else {
            None
        }
    }
}

impl Iterator for PageFrameIter<false> {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let frame = self.start;
            self.start += 1;
            Some(frame)
        } else {
            None
        }
    }
}

impl Add<u64> for Frame {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        Frame::which_contains(self.start() + rhs * bks::PAGE_SIZE)
    }
}

impl AddAssign<u64> for Frame {
    #[inline]
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}

impl Sub<u64> for Frame {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u64) -> Self::Output {
        Frame::which_contains(self.start() - rhs * bks::PAGE_SIZE)
    }
}

impl SubAssign<u64> for Frame {
    #[inline]
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

impl Sub<Frame> for Frame {
    type Output = u64;
    #[inline]
    fn sub(self, rhs: Frame) -> Self::Output {
        (self.start - rhs.start) / bks::PAGE_SIZE
    }
}
