use std::ops::{Add, AddAssign};

use crate::bigint::Uint256;

macro_rules! impl_add {
    ($($t:ty)*) => {
        $(
            impl Add<$t> for Uint256 {
                type Output = Uint256;

                fn add(self, rhs: $t) -> Self::Output {
                    Uint256(self.inner() + rhs)
                }
            }

            impl Add<$t> for &Uint256 {
                type Output = Uint256;

                fn add(self, rhs: $t) -> Self::Output {
                    Uint256(self.inner() + rhs)
                }
            }

            impl AddAssign<$t> for Uint256 {
                fn add_assign(&mut self, rhs: $t) {
                    *self = &*self + rhs;
                }
            }
        )*
    };
}

impl_add!(u8 u16 u32 u64 u128 usize);

impl Add<Uint256> for Uint256 {
    type Output = Uint256;

    fn add(self, rhs: Uint256) -> Self::Output {
        Self(self.inner() + rhs.inner())
    }
}

impl Add<Uint256> for &Uint256 {
    type Output = Uint256;

    fn add(self, other: Uint256) -> Self::Output {
        (self.inner() + other.inner()).into()
    }
}

impl Add<&Uint256> for Uint256 {
    type Output = Uint256;

    fn add(self, other: &Uint256) -> Self::Output {
        (self.inner() + other.inner()).into()
    }
}

impl Add<&Uint256> for &Uint256 {
    type Output = Uint256;

    fn add(self, other: &Uint256) -> Self::Output {
        (self.inner() + other.inner()).into()
    }
}

impl AddAssign<Uint256> for Uint256 {
    fn add_assign(&mut self, other: Uint256) {
        *self = &*self + other;
    }
}

impl AddAssign<&Uint256> for Uint256 {
    fn add_assign(&mut self, other: &Uint256) {
        *self = &*self + other;
    }
}
