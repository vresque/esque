use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ops::Sub;
use core::ops::SubAssign;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpperBound<const CAP: i128> {
    inner: i128,
}

impl<const CAP: i128> UpperBound<CAP> {
    pub fn new<T>(num: T) -> Self
    where
        T: Into<i128>,
    {
        let transformed = num.into();
        Self {
            inner: (if transformed > CAP { CAP } else { transformed }),
        }
    }

    pub fn cap(num: i128) -> i128 {
        if num > CAP {
            CAP
        } else {
            num
        }
    }
}

impl<const CAP: i128> Deref for UpperBound<CAP> {
    type Target = i128;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<const CAP: i128> DerefMut for UpperBound<CAP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<const CAP: i128> core::fmt::Debug for UpperBound<CAP> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("UpperBound")
            .field(&format_args!("{}", self.inner))
            .finish()
    }
}

impl<const CAP: i128> core::fmt::Binary for UpperBound<CAP> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.inner, f)
    }
}

impl<const CAP: i128> core::fmt::LowerHex for UpperBound<CAP> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl<const CAP: i128> core::fmt::Octal for UpperBound<CAP> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.inner, f)
    }
}

impl<const CAP: i128> core::fmt::UpperHex for UpperBound<CAP> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl<const CAP: i128> Add<u64> for UpperBound<CAP> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        UpperBound::<CAP>::new(self.inner + rhs as i128)
    }
}

impl<const CAP: i128> AddAssign<u64> for UpperBound<CAP> {
    #[inline]
    fn add_assign(&mut self, rhs: u64) {
        self.inner = self.inner + rhs as i128;
    }
}

impl<const CAP: i128> Add<usize> for UpperBound<CAP> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        self + rhs
    }
}

impl<const CAP: i128> AddAssign<usize> for UpperBound<CAP> {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.add_assign(rhs as u64)
    }
}

impl<const CAP: i128> Sub<u64> for UpperBound<CAP> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u64) -> Self::Output {
        UpperBound::new(self.inner - rhs as i128)
    }
}

impl<const CAP: i128> SubAssign<u64> for UpperBound<CAP> {
    #[inline]
    fn sub_assign(&mut self, rhs: u64) {
        self.inner = UpperBound::<CAP>::cap(self.inner - rhs as i128);
    }
}

impl<const CAP: i128> Sub<usize> for UpperBound<CAP> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        self - rhs
    }
}

impl<const CAP: i128> SubAssign<usize> for UpperBound<CAP> {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.sub_assign(rhs as usize)
    }
}

impl<const CAP: i128> Sub<UpperBound<CAP>> for UpperBound<CAP> {
    type Output = i128;
    #[inline]
    fn sub(self, rhs: UpperBound<CAP>) -> Self::Output {
        self.inner - rhs.inner
    }
}
