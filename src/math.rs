//! This module defines mathematical structures and operations, primarily relating
//! to complex numbers.

// The project is being worked on, and not all operations defined in this file are
// used anywhere yet. Therefore we allow "dead code".
#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// The primary type of real numbers. This may be switched to double precision if
/// necessary.
pub type Num = f32;

/// The pi constant. If necessary, can be defined by hand later.
pub const PI: Num = std::f32::consts::PI;

/// A complex number.
#[derive(Copy, Clone)]
pub struct Complex {
    pub real: Num,
    pub imag: Num,
}

impl Complex {
    /// The complex conjugate.
    pub fn conj(self) -> Self {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }

    /// Square of the absolute value.
    pub fn abs2(self) -> Num {
        self.real * self.real + self.imag * self.imag
    }

    /// Absolute value.
    pub fn abs(self) -> Num {
        self.abs2().sqrt()
    }

    /// Euler's formula,
    /// `e^(ix) = cos x + i sin x`.
    pub fn euler(x: Num) -> Self {
        Complex {
            real: x.cos(),
            imag: x.sin(),
        }
    }
}

/// Defines a conversion from a pair of real numbers into a complex number.
impl From<(Num, Num)> for Complex {
    fn from(pair: (Num, Num)) -> Complex {
        Complex {
            real: pair.0,
            imag: pair.1,
        }
    }
}

/// Displays a complex number in text form, akin to "(a + bi)".
impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag >= 0. {
            write!(f, "({} + {}i)", self.real, self.imag)
        } else {
            write!(f, "({} - {}i)", self.real, -self.imag)
        }
    }
}

// Basic mathematical operators are defined for complex numbers below.

impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Mul<Num> for Complex {
    type Output = Self;
    fn mul(self, other: Num) -> Self::Output {
        Complex {
            real: self.real * other,
            imag: self.imag * other,
        }
    }
}

impl Div<Num> for Complex {
    type Output = Self;
    fn div(self, other: Num) -> Self::Output {
        Complex {
            real: self.real / other,
            imag: self.imag / other,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        let dividend = Complex {
            real: self.real * other.real + self.imag * other.imag,
            imag: self.imag * other.real - self.real * other.imag,
        };
        let divisor = other.abs2();
        dividend / divisor
    }
}
