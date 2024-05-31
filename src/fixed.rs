use std::ops::{Add, AddAssign, Div, Mul, Sub};

/// Fixed precision signed integer with 8 bits of fractional precision
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedI32(i32);

impl FixedI32 {
    pub const ZERO: Self = Self(0);
    pub const NEGATIVE_ONE: Self = Self(-1);
    pub const HALF_PIXEL: Self = Self(128);

    pub fn from_f32_lossy(x: f32) -> Self {
        Self((x * 256.0).round() as i32)
    }

    pub fn to_i32_lossy(&self) -> i32 {
        self.0 / 256
    }

    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn min(&self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    pub fn max(&self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }
}

impl From<FixedI32> for f32 {
    fn from(value: FixedI32) -> Self {
        value.0 as f32 * 256.0
    }
}

impl From<i32> for FixedI32 {
    fn from(value: i32) -> Self {
        Self(value * 256)
    }
}

impl From<u16> for FixedI32 {
    fn from(value: u16) -> Self {
        Self::from(value as i32)
    }
}

impl Add<FixedI32> for FixedI32 {
    type Output = FixedI32;

    fn add(self, rhs: FixedI32) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<FixedI32> for FixedI32 {
    fn add_assign(&mut self, rhs: FixedI32) {
        *self = *self + rhs
    }
}

impl Sub<FixedI32> for FixedI32 {
    type Output = FixedI32;

    fn sub(self, rhs: FixedI32) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<FixedI32> for FixedI32 {
    type Output = FixedI32;

    fn mul(self, rhs: FixedI32) -> Self::Output {
        let product = (self.0 as i64 * rhs.0 as i64) / 256;

        Self(product as i32)
    }
}

impl Div<FixedI32> for FixedI32 {
    type Output = FixedI32;

    fn div(self, rhs: FixedI32) -> Self::Output {
        let quotient = self.0 as i64 * 256 / rhs.0 as i64;

        Self(quotient as i32)
    }
}
