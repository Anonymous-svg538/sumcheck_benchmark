use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct F17(u64);

pub const MODULUS: u64 = 17;

impl F17 {
    pub fn new(value: u64) -> Self {
        Self(value % MODULUS)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn one() -> Self {
        Self(1)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl Add for F17 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        F17::new(self.0 + rhs.0)
    }
}

impl Sub for F17 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        F17::new((self.0 + MODULUS - rhs.0) % MODULUS)
    }
}

impl Mul for F17 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        F17::new(self.0 * rhs.0)
    }
}

impl fmt::Display for F17 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}