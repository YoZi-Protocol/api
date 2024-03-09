use num_bigint::BigUint;
pub use num_traits::{FromBytes, FromPrimitive, Num, One, Pow, ToBytes, ToPrimitive, Zero};

mod add;
mod div;
mod mul;
mod sub;

mod fmt;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Uint256(BigUint);

impl Uint256 {
    pub fn max() -> Self {
        Self(BigUint::from_bytes_be(&[0xff; 32]))
    }

    pub fn from_str_prefixed(s: &str) -> Result<Self, ()> {
        if let Some(hex) = s.strip_prefix("0x") {
            BigUint::from_str_radix(hex, 16).map(Self).map_err(|_| ())
        } else {
            BigUint::from_str_radix(s, 10).map(Self).map_err(|_| ())
        }
    }

    pub fn inner(&self) -> &BigUint {
        &self.0
    }
}

impl From<BigUint> for Uint256 {
    fn from(value: BigUint) -> Self {
        Self(value)
    }
}

impl From<&BigUint> for Uint256 {
    fn from(value: &BigUint) -> Self {
        Self(value.clone())
    }
}

impl Zero for Uint256 {
    fn zero() -> Self {
        Self(BigUint::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for Uint256 {
    fn one() -> Self {
        Self(BigUint::one())
    }
}

impl Pow<i32> for Uint256 {
    type Output = Self;

    fn pow(self, exp: i32) -> Self::Output {
        if exp == 0 {
            return Uint256::one();
        }

        Pow::pow(self, exp)
    }
}

impl Default for Uint256 {
    fn default() -> Self {
        Self::zero()
    }
}

impl FromBytes for Uint256 {
    type Bytes = [u8; 32];

    fn from_be_bytes(bytes: &Self::Bytes) -> Self {
        Self(BigUint::from_be_bytes(bytes))
    }

    fn from_le_bytes(bytes: &Self::Bytes) -> Self {
        Self(BigUint::from_le_bytes(bytes))
    }
}

impl ToBytes for Uint256 {
    type Bytes = [u8; 32];

    fn to_be_bytes(&self) -> Self::Bytes {
        let bytes = self.inner().to_be_bytes();

        let mut result = [0; 32];
        result[32 - bytes.len()..].copy_from_slice(&bytes);

        result
    }

    fn to_le_bytes(&self) -> Self::Bytes {
        let bytes = self.inner().to_le_bytes();

        let mut result = [0; 32];
        result[..bytes.len()].copy_from_slice(&bytes);

        result
    }
}

impl FromPrimitive for Uint256 {
    fn from_i64(n: i64) -> Option<Self> {
        BigUint::from_i64(n).map(Self)
    }

    fn from_i128(n: i128) -> Option<Self> {
        BigUint::from_i128(n).map(Self)
    }

    fn from_u64(n: u64) -> Option<Self> {
        BigUint::from_u64(n).map(Self)
    }

    fn from_u128(n: u128) -> Option<Self> {
        BigUint::from_u128(n).map(Self)
    }
}

impl ToPrimitive for Uint256 {
    fn to_i64(&self) -> Option<i64> {
        self.inner().to_i64()
    }

    fn to_i128(&self) -> Option<i128> {
        self.inner().to_i128()
    }

    fn to_u64(&self) -> Option<u64> {
        self.inner().to_u64()
    }

    fn to_u128(&self) -> Option<u128> {
        self.inner().to_u128()
    }

    fn to_f32(&self) -> Option<f32> {
        self.inner().to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.inner().to_f64()
    }
}
