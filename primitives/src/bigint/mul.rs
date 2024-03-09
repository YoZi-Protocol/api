use std::ops::{Mul, MulAssign};

use crate::bigint::Uint256;

macro_rules! impl_mul {
    ($($t:ty)*) => {
        $(
            impl Mul<$t> for Uint256 {
                type Output = Uint256;

                fn mul(self, rhs: $t) -> Self::Output {
                    Uint256(self.inner() * rhs)
                }
            }

            impl Mul<$t> for &Uint256 {
                type Output = Uint256;

                fn mul(self, rhs: $t) -> Self::Output {
                    Uint256(self.inner() * rhs)
                }
            }

            impl MulAssign<$t> for Uint256 {
                fn mul_assign(&mut self, rhs: $t) {
                    *self = &*self * rhs;
                }
            }
        )*
    };
}

impl_mul!(u8 u16 u32 u64 u128 usize);

impl Mul<Uint256> for Uint256 {
    type Output = Uint256;

    fn mul(self, rhs: Uint256) -> Self::Output {
        Self(self.inner() * rhs.inner())
    }
}

impl Mul<Uint256> for &Uint256 {
    type Output = Uint256;

    fn mul(self, other: Uint256) -> Self::Output {
        (self.inner() * other.inner()).into()
    }
}

impl Mul<&Uint256> for Uint256 {
    type Output = Uint256;

    fn mul(self, other: &Uint256) -> Self::Output {
        (self.inner() * other.inner()).into()
    }
}

impl Mul<&Uint256> for &Uint256 {
    type Output = Uint256;

    fn mul(self, other: &Uint256) -> Self::Output {
        (self.inner() * other.inner()).into()
    }
}

impl MulAssign<Uint256> for Uint256 {
    fn mul_assign(&mut self, other: Uint256) {
        *self = &*self * other;
    }
}

impl MulAssign<&Uint256> for Uint256 {
    fn mul_assign(&mut self, other: &Uint256) {
        *self = &*self * other;
    }
}
