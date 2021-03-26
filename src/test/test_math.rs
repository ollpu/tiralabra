
use crate::math::*;
use super::util::*;

#[test]
fn abs_of_1_plus_i_is_sqrt_2() {
    let complex = Complex { real: 1., imag: 1. };
    let abs = complex.abs();
    assert!(float_eq(abs, 2f32.sqrt(), 6));
}

#[test]
fn simple_complex_multiplication() {
    let a: Complex = Complex { real: 1., imag: 1. };
    let b: Complex = Complex { real: -1., imag: 1. };
    let product = a * b;
    assert!(complex_eq(product, (-2., 0.), 6));
}

#[test]
fn simple_complex_division() {
    let a = Complex { real: 1., imag: 2. };
    let b = Complex { real: -1., imag: 1. };
    let quotient = a / b;
    assert!(complex_eq(quotient, (0.5, -1.5), 6));
}

#[test]
fn simple_euler() {
    let z = Complex::euler(PI / 4.);
    assert!(complex_eq(z, (1./2f32.sqrt(), 1./2f32.sqrt()), 6));
}
