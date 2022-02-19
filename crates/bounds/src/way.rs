use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ops::Sub;
use core::ops::SubAssign;

use crate::upper;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoWayBound<const LOWER: i128, const UPPER: i128, const DEFAULT: i128 = -2303282> {
    inner: i128,
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> TwoWayBound<LOWER, UPPER, DEFAULT> {
    pub fn new<T>(num: T) -> Self
    where
        T: Into<i128>,
    {
        Self {
            inner: Self::cap(num),
        }
    }

    pub fn cap<T>(num: T) -> i128
    where
        T: Into<i128>,
    {
        let transformed = num.into();
        let (upper_default, lower_default) = if DEFAULT == -2303282 {
            (UPPER, LOWER)
        } else {
            (DEFAULT, DEFAULT)
        };
        if transformed > UPPER {
            upper_default
        } else if transformed < LOWER {
            lower_default
        } else {
            transformed
        }
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> Deref
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    type Target = i128;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> DerefMut
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> core::fmt::Debug
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("TwoWayBound")
            .field(&format_args!("{}", self.inner))
            .finish()
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> core::fmt::Binary
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.inner, f)
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> core::fmt::LowerHex
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> core::fmt::Octal
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.inner, f)
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> core::fmt::UpperHex
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> Add<u64>
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    type Output = Self;
    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        TwoWayBound::<LOWER, UPPER, DEFAULT>::new(self.inner + rhs as i128)
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> AddAssign<u64>
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    #[inline]
    fn add_assign(&mut self, rhs: u64) {
        self.inner = self.inner + rhs as i128;
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> Add<usize>
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    type Output = Self;
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        self + rhs
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> AddAssign<usize>
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.add_assign(rhs as u64)
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> Sub<u64>
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u64) -> Self::Output {
        TwoWayBound::new(self.inner - rhs as i128)
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> SubAssign<u64>
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    #[inline]
    fn sub_assign(&mut self, rhs: u64) {
        self.inner = TwoWayBound::<LOWER, UPPER, DEFAULT>::cap(self.inner - rhs as i128);
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> Sub<usize>
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    type Output = Self;
    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        self - rhs
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128> SubAssign<usize>
    for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.sub_assign(rhs as usize)
    }
}

impl<const LOWER: i128, const UPPER: i128, const DEFAULT: i128>
    Sub<TwoWayBound<LOWER, UPPER, DEFAULT>> for TwoWayBound<LOWER, UPPER, DEFAULT>
{
    type Output = i128;
    #[inline]
    fn sub(self, rhs: TwoWayBound<LOWER, UPPER, DEFAULT>) -> Self::Output {
        self.inner - rhs.inner
    }
}
