use std::ops::{Sub, SubAssign};

use crate::bigint::Uint256;

macro_rules! impl_sub {
    ($($t:ty)*) => {
        $(
            impl Sub<$t> for Uint256 {
                type Output = Uint256;

                fn sub(self, rhs: $t) -> Self::Output {
                    Uint256(self.inner() - rhs)
                }
            }

            impl Sub<$t> for &Uint256 {
                type Output = Uint256;

                fn sub(self, rhs: $t) -> Self::Output {
                    Uint256(self.inner() - rhs)
                }
            }

            impl SubAssign<$t> for Uint256 {
                fn sub_assign(&mut self, rhs: $t) {
                    *self = &*self - rhs;
                }
            }
        )*
    };
}

impl_sub!(u8 u16 u32 u64 u128 usize);

impl Sub<Uint256> for Uint256 {
    type Output = Uint256;

    fn sub(self, rhs: Uint256) -> Self::Output {
        Self(self.inner() - rhs.inner())
    }
}

impl Sub<Uint256> for &Uint256 {
    type Output = Uint256;

    fn sub(self, other: Uint256) -> Self::Output {
        (self.inner() - other.inner()).into()
    }
}

impl Sub<&Uint256> for Uint256 {
    type Output = Uint256;

    fn sub(self, other: &Uint256) -> Self::Output {
        (self.inner() - other.inner()).into()
    }
}

impl Sub<&Uint256> for &Uint256 {
    type Output = Uint256;

    fn sub(self, other: &Uint256) -> Self::Output {
        (self.inner() - other.inner()).into()
    }
}

impl SubAssign<Uint256> for Uint256 {
    fn sub_assign(&mut self, other: Uint256) {
        *self = &*self - other;
    }
}

impl SubAssign<&Uint256> for Uint256 {
    fn sub_assign(&mut self, other: &Uint256) {
        *self = &*self - other;
    }
}
