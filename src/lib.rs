#![no_std]

use core::fmt;
use core::ops::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct DyadicFraction {
    num: i32,
    power: u8,
}

impl DyadicFraction {
    pub const fn new(num: i32, power: u8) -> Self {
        Self { num, power }
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

    pub const fn normalize(self) -> Self {
        let mut res = self;
        while (res.num & 1) == 0 && res.power > 0 {
            res.power -= 1;
            res.num >>= 1;
        }
        res
    }

    pub fn num(&self) -> i32 {
        self.num
    }

    pub fn power(&self) -> u8 {
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
        self.num.shr(self.power)
    }
}

impl AddAssign for DyadicFraction {
    fn add_assign(&mut self, rhs: Self) {
        let (min_power, max_power) = if self.power > rhs.power {
            (rhs.power, self.power)
        } else {
            (self.power, rhs.power)
        };
        self.num = self.num.shl(rhs.power - min_power) + rhs.num.shl(self.power - min_power);
        self.power = max_power;
    }
}

impl Add for DyadicFraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = self;
        res += other;
        res
    }
}

impl SubAssign for DyadicFraction {
    fn sub_assign(&mut self, rhs: Self) {
        let (min_power, max_power) = if self.power > rhs.power {
            (rhs.power, self.power)
        } else {
            (self.power, rhs.power)
        };
        self.num = self.num.shl(rhs.power - min_power) - rhs.num.shl(self.power - min_power);
        self.power = max_power;
    }
}

impl Sub for DyadicFraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = self;
        res -= other;
        res
    }
}

impl MulAssign for DyadicFraction {
    fn mul_assign(&mut self, rhs: Self) {
        self.num *= rhs.num;
        self.power += rhs.power;
    }
}

impl Mul for DyadicFraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut res = self;
        res *= other;
        res
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
        let rhs = self.normalize();
        let lhs = other.normalize();
        rhs.power == lhs.power && rhs.num == lhs.num
    }
}

impl Eq for DyadicFraction {}

impl fmt::Display for DyadicFraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.power == 0 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, 1.shl(self.power))
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
fn test_normalize() {
    let a = DyadicFraction::new(80, 3).normalize();
    assert_eq!(10, a.into())
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
    assert!(a == b)
}
