#![no_std]

use core::cmp::*;
use core::fmt;
use core::ops::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct DyadicFraction {
    num: i32,
    power: i8,
}

impl DyadicFraction {
    pub const fn new(num: i32, power: i8) -> Self {
        Self { num, power }
    }

    pub const fn zero() -> Self {
        Self { num: 0, power: 0 }
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

    pub const fn abs(self) -> Self {
        Self {
            num: self.num.abs(),
            power: self.power,
        }
    }

    pub const fn signum(self) -> Self {
        Self {
            num: self.num.signum(),
            power: 0,
        }
    }

    pub const fn is_positive(self) -> bool {
        self.num.is_positive()
    }

    pub const fn is_negative(self) -> bool {
        self.num.is_negative()
    }

    pub const fn canonical(&self) -> Self {
        let mut res = *self;
        while (res.num & 1) == 0 {
            res.power -= 1;
            res.num >>= 1;
        }
        res
    }

    pub fn num(&self) -> i32 {
        self.num
    }

    pub fn power(&self) -> i8 {
        self.power
    }
}

impl From<i32> for DyadicFraction {
    fn from(num: i32) -> Self {
        Self { num, power: 0 }
    }
}

impl From<i16> for DyadicFraction {
    fn from(num: i16) -> Self {
        Self {
            num: num as _,
            power: 0,
        }
    }
}

impl From<u16> for DyadicFraction {
    fn from(num: u16) -> Self {
        Self {
            num: num as _,
            power: 0,
        }
    }
}

impl From<u8> for DyadicFraction {
    fn from(num: u8) -> Self {
        Self {
            num: num as _,
            power: 0,
        }
    }
}

impl From<i8> for DyadicFraction {
    fn from(num: i8) -> Self {
        Self {
            num: num as _,
            power: 0,
        }
    }
}

impl Into<i32> for DyadicFraction {
    fn into(self) -> i32 {
        if self.power.is_negative() {
            self.num.shl(self.power.abs())
        } else {
            self.num.shr(self.power)
        }
    }
}

impl Add for DyadicFraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (min_power, max_power) = if self.power > other.power {
            (other.power, self.power)
        } else {
            (self.power, other.power)
        };
        let power = max_power;
        let num = self
            .num
            .shl(other.power - min_power)
            .saturating_add(other.num.shl(self.power - min_power));
        Self::new(num, power).canonical()
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
        let (min_power, max_power) = if self.power > other.power {
            (other.power, self.power)
        } else {
            (self.power, other.power)
        };
        let power = max_power;
        let num = self
            .num
            .shl(other.power - min_power)
            .saturating_sub(other.num.shl(self.power - min_power));
        Self::new(num, power).canonical()
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
        .canonical()
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
        self * Self::from(-1)
    }
}

impl PartialEq for DyadicFraction {
    fn eq(&self, other: &Self) -> bool {
        let val: i32 = (*self).into();
        val.eq(&(*other).into())
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
        let lhs = if other.power.is_negative() {
            self.num.shr(other.power.abs())
        } else {
            self.num.shl(other.power)
        };

        let rhs = if self.power.is_negative() {
            other.num.shr(self.power.abs())
        } else {
            other.num.shl(self.power)
        };

        lhs.cmp(&rhs)
    }
}

impl fmt::Display for DyadicFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.power > 0 {
            write!(f, "{}/{}", self.num, 1.shl(self.power))
        } else {
            write!(f, "{}", self.num.shl(self.power.abs()))
        }
    }
}

#[test]
fn test_add() {
    let a = DyadicFraction::from(2);
    let b = DyadicFraction::new(4, 3);
    let c = a + b;
    let d = c * 1000.into();
    assert_eq!(2500, d.into())
}

#[test]
fn test_sub() {
    let a = DyadicFraction::from(2);
    let b = DyadicFraction::new(4, 3);
    let c = a - b;
    let d = c * 1000.into();
    assert_eq!(1500, d.into())
}

#[test]
fn test_mul() {
    let mut a = DyadicFraction::new(3, 2);
    a *= 100.into();
    assert_eq!(75, a.into())
}

#[test]
fn test_canonical() {
    let a = DyadicFraction::new(80, 3);
    assert_eq!(10, a.into());
    assert_eq!(DyadicFraction::new(5, -1), a.canonical());
}

#[test]
fn test_neg() {
    let a = DyadicFraction::new(3, 2);
    let b = DyadicFraction::new(-3, 2);
    assert_eq!(a, -b)
}

#[test]
fn test_abs() {
    let a = DyadicFraction::new(3, 2);
    let b = DyadicFraction::new(-3, 2);
    assert_eq!(a, b.abs())
}

#[test]
fn test_eq() {
    let a = DyadicFraction::new(4, 3);
    let b = DyadicFraction::new(8, 4);
    assert_eq!(a, b)
}

#[test]
fn test_cmp() {
    let a = DyadicFraction::new(4, 3);
    let b = DyadicFraction::new(7, 4);
    assert!(a > b)
}

#[test]
fn test_max() {
    let a = DyadicFraction::new(4, 3);
    let b = DyadicFraction::new(7, 4);
    let min = DyadicFraction::min(a, b);
    let max = DyadicFraction::max(a, b);
    assert_eq!(a, max);
    assert_eq!(b, min);
}
