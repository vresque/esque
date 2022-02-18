#![no_std]

#[macro_export]
macro_rules! num_backed {
    ($visi:vis $name:ident backed by $backend:ident) => {
        $visi struct $name($backend);

        impl $name {
            #[allow(dead_code)]
            pub const fn inner(self) -> $backend { self.0 }
        }

        impl Into<$backend> for $name {
            fn into(self) -> $backend { self.0 }
        }

        impl From<$backend> for $name {
            fn from(bck: $backend) -> Self { Self(bck) }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&format_args!("{:#x}", self.0))
                    .finish()
            }
        }
        
        impl core::fmt::Binary for $name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Binary::fmt(&self.0, f)
            }
        }
        
        impl core::fmt::LowerHex for $name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::LowerHex::fmt(&self.0, f)
            }
        }
        
        impl core::fmt::Octal for $name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Octal::fmt(&self.0, f)
            }
        }
        
        impl core::fmt::UpperHex for $name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::UpperHex::fmt(&self.0, f)
            }
        }
        
        impl core::fmt::Pointer for $name {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Pointer::fmt(&(self.0 as *const ()), f)
            }
        }
        
        impl Add<$backend> for $name {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $backend) -> Self::Output {
                $name::new(self.0 + rhs)
            }
        }
        
        impl AddAssign<$backend> for $name {
            #[inline]
            fn add_assign(&mut self, rhs: $backend) {
                *self = *self + rhs;
            }
        }
        
        impl Add<usize> for $name {
            type Output = Self;
            #[inline]
            fn add(self, rhs: usize) -> Self::Output {
                self + rhs as $backend
            }
        }
        
        impl AddAssign<usize> for $name {
            #[inline]
            fn add_assign(&mut self, rhs: usize) {
                self.add_assign(rhs as $backend)
            }
        }
        
        impl Sub<$backend> for $name {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: $backend) -> Self::Output {
                $name::new(self.0.checked_sub(rhs).unwrap())
            }
        }
        
        impl SubAssign<$backend> for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: $backend) {
                *self = *self - rhs;
            }
        }
        
        impl Sub<usize> for $name {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: usize) -> Self::Output {
                self - rhs as $backend
            }
        }
        
        impl SubAssign<usize> for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: usize) {
                self.sub_assign(rhs as $backend)
            }
        }
        
        impl Sub<$name> for $name {
            type Output = $backend;
            #[inline]
            fn sub(self, rhs: $name) -> Self::Output {
                self.as_u64().checked_sub(rhs.as_u64()).unwrap()
            }
        }
        
    };

    ($visi:vis $name:ident backed by $backend:ident;
    atomic: $atomic_visi:vis $atomic_name:ident backed by $atomic_backend:ident) => {
        num_backed::num_backed!($visi $name backed by $backend);

        // A holder for T that can be shared among threads
        $atomic_visi struct $atomic_name {
            container: $atomic_backend,
        }

        impl $atomic_name {
            #[allow(dead_code)]
            pub const fn new(x: $name) -> Self {
                Self { container: $atomic_backend::new(x.into()) }
            }
            #[allow(dead_code)]
            pub fn compare_exchange_weak(&self, current: $name, new: $name, order_on_success: ::core::sync::atomic::Ordering, order_on_failure: ::core::sync::atomic::Ordering) -> Result<$name, $name> {
                match self.container.compare_exchange_weak(current.into(), new.into(), order_on_success, order_on_failure) {
                    Ok(good) => Ok($name::from(good)),
                    Err(bad) => Err($name:.from(bad)),
                }
            }

            #[allow(dead_code)]
            pub fn compare_exchange(&self, current: $name, new: $name, order_on_success: ::core::sync::atomic::Ordering, order_on_failure: ::core::sync::atomic::Ordering) -> Reuslt<$name, $name> {
                match self.container.compare_exchange(current.into(), new.into(), order_on_success, order_on_failure) {
                    Ok(good) => Ok($name::from(good)),
                    Err(bad) => Err($name::from(bad)),
                }
            }

            #[allow(dead_code)]
            pub fn store(&self, value: $name, order: ::core::sync::atomic::Ordering) {
                self.container.store(value.into(), order);
            }

            #[allow(dead_code)]
            pub fn load(&self, order: ::core::sync::atomic::Ordering) -> $name {
                $name::from(self.container.load(order))
            }

            #[allow(dead_code)]
            pub fn swap(&self, value: $name, order: ::core::sync::atomic::Ordering) -> $name {
                $name::from(self.container.swap(value.into(), order))
            }
        }

        impl Default for $atomic_name {
            fn default() -> Self { Self::new($backend::from(0) ) }
        }
        
    }
}