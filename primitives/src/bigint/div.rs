use std::ops::{Div, DivAssign};

use crate::bigint::Uint256;

macro_rules! impl_div {
    ($($t:ty)*) => {
        $(
            impl Div<$t> for Uint256 {
                type Output = Uint256;

                fn div(self, rhs: $t) -> Self::Output {
                    Uint256(self.inner() / rhs)
                }
            }

            impl Div<$t> for &Uint256 {
                type Output = Uint256;

                fn div(self, rhs: $t) -> Self::Output {
                    Uint256(self.inner() / rhs)
                }
            }

            impl DivAssign<$t> for Uint256 {
                fn div_assign(&mut self, rhs: $t) {
                    *self = &*self / rhs;
                }
            }
        )*
    };
}

impl_div!(u8 u16 u32 u64 u128 usize);

impl Div<Uint256> for Uint256 {
    type Output = Uint256;

    fn div(self, rhs: Uint256) -> Self::Output {
        Self(self.inner() / rhs.inner())
    }
}

impl Div<Uint256> for &Uint256 {
    type Output = Uint256;

    fn div(self, other: Uint256) -> Self::Output {
        (self.inner() / other.inner()).into()
    }
}

impl Div<&Uint256> for Uint256 {
    type Output = Uint256;

    fn div(self, other: &Uint256) -> Self::Output {
        (self.inner() / other.inner()).into()
    }
}

impl Div<&Uint256> for &Uint256 {
    type Output = Uint256;

    fn div(self, other: &Uint256) -> Self::Output {
        (self.inner() / other.inner()).into()
    }
}

impl DivAssign<Uint256> for Uint256 {
    fn div_assign(&mut self, other: Uint256) {
        *self = &*self / other;
    }
}

impl DivAssign<&Uint256> for Uint256 {
    fn div_assign(&mut self, other: &Uint256) {
        *self = &*self / other;
    }
}
