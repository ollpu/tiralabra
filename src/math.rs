//! Tässä moduulissa toteutetaan matemaattisia operaatiota, ja määritellään
//! reaaliluvuille sekä kompleksiluvuille tyypit.

// Projekti on kesken, eikä kaikkia toteutettuja operaatioita vielä käytetä missään.
// Sallitaan siis "kuollut koodi".
#![allow(dead_code)]

use std::ops::{Add, Sub, Neg, Mul, Div};
use std::fmt;

/// Lukujen tyyppi. Tästä voidaan tarvittaessa vaihtaa tuplatarkkuuteen (f64).
pub type Num = f32;

/// Pii-vakio. Tarvittaessa voidaan määritellä käsin.
pub const PI: Num = std::f32::consts::PI;

/// Kompleksiluku.
#[derive(Copy, Clone)]
pub struct Complex {
    pub real: Num,
    pub imag: Num,
}

impl Complex {
    /// Kompleksi konjugaatti.
    pub fn conj(self) -> Self {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }

    /// Itseisarvon neliö.
    pub fn abs2(self) -> Num {
        self.real * self.real + self.imag * self.imag
    }

    /// Itseisarvo.
    pub fn abs(self) -> Num {
        self.abs2().sqrt()
    }

    /// Eulerin kaava eli
    /// `e^(ix) = cos x + i sin x`.
    pub fn euler(x: Num) -> Self {
        Complex {
            real: x.cos(),
            imag: x.sin(),
        }
    }
}

/// Määrittelee muunnoksen parista lukuja kompleksiluvuksi.
impl From<(Num, Num)> for Complex {
    fn from(pair: (Num, Num)) -> Complex {
        Complex {
            real: pair.0,
            imag: pair.1,
        }
    }
}

/// Esittää kompleksiluvun tekstimuodossa, jotakuinkin "(a + bi)".
impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag >= 0. {
            write!(f, "({} + {}i)", self.real, self.imag)
        } else {
            write!(f, "({} - {}i)", self.real, -self.imag)
        }
    }
}

// Alla määritellään peruslaskutoimitukset kompleksiluvuille.

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
        let jaettava = Complex {
            real: self.real * other.real + self.imag * other.imag,
            imag: self.imag * other.real - self.real * other.imag,
        };
        let jakaja = other.abs2();
        jaettava / jakaja
    }
}
