#![no_std]

use core::cmp::*;
use core::fmt;
use core::ops::*;

pub type DF = DyadicFraction;

#[derive(Copy, Clone, Debug, Default)]
pub struct DyadicFraction {
    num: i32,
    power: i8,
}

impl DyadicFraction {
    pub const fn new(numerator: i32, denominator_power: i8) -> Self {
        let df = Self {
            num: numerator,
            power: denominator_power,
        };
        df.canonical()
    }

    pub const fn zero() -> Self {
        Self { num: 0, power: 0 }
    }

    pub const fn abs(self) -> Self {
        Self::new(self.num.abs(), self.power)
    }

    pub const fn signum(self) -> Self {
        Self::new(self.num.signum(), 0)
    }

    pub const fn copysign(self, sign: i32) -> Self {
        if sign.signum() == self.num.signum() {
            return self;
        }
        Self::new(-self.num, self.power)
    }

    pub const fn is_positive(self) -> bool {
        self.num.is_positive()
    }

    pub const fn is_negative(self) -> bool {
        self.num.is_negative()
    }

    pub const fn canonical(&self) -> Self {
        let mut res = *self;
        if res.num == 0 {
            res.power = 0;
        }
        while res.num != 0 && (res.num & 1) == 0 {
            res.power -= 1;
            res.num >>= 1;
        }
        res
    }

    pub const fn round(&self, denominator_power: i8) -> Self {
        let mut res = self.canonical();
        if denominator_power >= res.power {
            return res;
        }
        while res.power > denominator_power {
            res.num &= !1;
            res = res.canonical();
        }
        res
    }

    pub fn div_by_two(&self) -> Self {
        let mut res = *self;
        res.power += 1;
        res
    }

    pub fn mul_add(self, a: Self, b: Self) -> Self {
        self * a + b
    }

    pub fn scale(self, a: impl Into<Self>) -> i32 {
        self.canonical().mul(a.into()).into()
    }

    pub fn pow(self, n: u8) -> Self {
        if n == 0 {
            return self.signum();
        }
        let mut res = self;
        for _ in 0..n - 1 {
            res *= self;
        }
        res
    }

    pub fn max(lhs: Self, rhs: Self) -> Self {
        if lhs > rhs {
            lhs
        } else {
            rhs
        }
    }

    pub fn min(lhs: Self, rhs: Self) -> Self {
        if lhs < rhs {
            lhs
        } else {
            rhs
        }
    }

    pub fn numerator(&self) -> i32 {
        self.num
    }

    pub fn denominator_power(&self) -> i8 {
        self.power
    }

    fn saturating_cross(self, other: Self) -> (i32, i32, i8) {
        let (min_power, max_power) = if self.power > other.power {
            (other.power, self.power)
        } else {
            (self.power, other.power)
        };
        let power = max_power;
        let shift = other.power - min_power;
        let fst = if shift < 32 {
            self.num.shl(shift)
        } else if self.num.is_positive() {
            i32::MAX
        } else {
            i32::MIN
        };

        let shift = self.power - min_power;
        let snd = if shift < 32 {
            other.num.shl(shift)
        } else if other.is_positive() {
            i32::MAX
        } else {
            i32::MIN
        };
        (fst, snd, power)
    }
}

impl From<DyadicFraction> for i32 {
    fn from(val: DyadicFraction) -> Self {
        let val = val.canonical();
        let shift = val.power.abs();
        if shift > 32 {
            if val.power.is_negative() {
                val.num.signum() * i32::MAX
            } else {
                0
            }
        } else if val.power.is_negative() {
            val.num.shl(shift)
        } else {
            val.num.shr(shift)
        }
    }
}

impl From<i32> for DyadicFraction {
    fn from(num: i32) -> Self {
        Self::new(num, 0)
    }
}

impl From<isize> for DyadicFraction {
    fn from(num: isize) -> Self {
        Self::new(num as _, 0)
    }
}

impl From<i16> for DyadicFraction {
    fn from(num: i16) -> Self {
        Self::new(num as _, 0)
    }
}

impl From<u16> for DyadicFraction {
    fn from(num: u16) -> Self {
        Self::new(num as _, 0)
    }
}

impl From<u8> for DyadicFraction {
    fn from(num: u8) -> Self {
        Self::new(num as _, 0)
    }
}

impl From<i8> for DyadicFraction {
    fn from(num: i8) -> Self {
        Self::new(num as _, 0)
    }
}

impl Add for DyadicFraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (fst, snd, power) = self.saturating_cross(other);
        Self::new(fst.saturating_add(snd), power)
    }
}

impl AddAssign for DyadicFraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for DyadicFraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (fst, snd, power) = self.saturating_cross(other);
        Self::new(fst.saturating_sub(snd), power)
    }
}

impl SubAssign for DyadicFraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for DyadicFraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.num.saturating_mul(other.num),
            self.power.saturating_add(other.power),
        )
    }
}

impl MulAssign for DyadicFraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Neg for DyadicFraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.num, self.power)
    }
}

impl PartialEq for DyadicFraction {
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.canonical();
        let rhs = other.canonical();
        lhs.power == rhs.power && lhs.num == rhs.num
    }
}

impl Eq for DyadicFraction {}

impl PartialOrd for DyadicFraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DyadicFraction {
    fn cmp(&self, other: &Self) -> Ordering {
        let delta = (*self - *other).numerator().signum();
        if delta == 0 {
            Ordering::Equal
        } else if delta.is_positive() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl fmt::Display for DyadicFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.canonical();
        let shift = val.power.abs();
        if val.power == 0 || val.power.is_negative() {
            let val = if shift < 32 {
                val.num.shl(shift)
            } else if val.is_positive() {
                i32::MAX
            } else {
                i32::MIN
            };
            write!(f, "{}", val)
        } else {
            let den = if shift < 32 { 1.shl(shift) } else { i32::MAX };
            write!(f, "{}/{}", val.num, den)
        }
    }
}

pub mod consts {
    use super::*;

    /// The full circle constant (τ)
    ///
    /// Equal to 2π.
    pub const TAU: DyadicFraction = DyadicFraction::new(3217, 9);

    /// Archimedes' constant (π)
    pub const PI: DyadicFraction = DyadicFraction::new(3217, 10);

    /// π/2
    pub const FRAC_PI_2: DyadicFraction = DyadicFraction::new(3217, 11);

    /// π/4
    pub const FRAC_PI_4: DyadicFraction = DyadicFraction::new(3217, 12);

    /// π/3
    pub const FRAC_PI_3: DyadicFraction = DyadicFraction::new(68629, 16);

    /// 1/π
    pub const FRAC_1_PI: DyadicFraction = DyadicFraction::new(20861, 16);

    /// 2/π
    pub const FRAC_2_PI: DyadicFraction = DyadicFraction::new(20861, 15);

    /// 2/sqrt(π)
    pub const FRAC_2_SQRT_PI: DyadicFraction = DyadicFraction::new(73949, 16);

    /// sqrt(2)
    pub const SQRT_2: DyadicFraction = DyadicFraction::new(46341, 15);

    /// 1/sqrt(2)
    pub const FRAC_1_SQRT_2: DyadicFraction = DyadicFraction::new(46341, 16);

    /// Euler's number (e)
    pub const E: DyadicFraction = DyadicFraction::new(178145, 16);

    /// log<sub>2</sub>(10)
    pub const LOG2_10: DyadicFraction = DyadicFraction::new(108853, 15);

    /// log<sub>2</sub>(e)
    pub const LOG2_E: DyadicFraction = DyadicFraction::new(23637, 14);

    /// log<sub>10</sub>(2)
    pub const LOG10_2: DyadicFraction = DyadicFraction::new(1233, 12);

    /// log<sub>10</sub>(e)
    pub const LOG10_E: DyadicFraction = DyadicFraction::new(14231, 15);

    /// ln(2)
    pub const LN_2: DyadicFraction = DyadicFraction::new(22713, 15);

    /// ln(10)
    pub const LN_10: DyadicFraction = DyadicFraction::new(75451, 15);
}
