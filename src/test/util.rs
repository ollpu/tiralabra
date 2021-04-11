use crate::math::{Complex, Num};

/// Ensures that `a` differs from the number `b` by at most `precision` significant
/// digits. `b` is the "correct answer", so the relative error is computed based on
/// its magnitude. When `b` < 1, absolute error is used instead.
pub fn float_eq(a: Num, b: Num, precision: u32) -> bool {
    // We convert the numbers into double precision first, so that no further
    // errors are introduced while comparing.
    let a = a as f64;
    let b = b as f64;
    let relative_error = (a - b).abs() / b.abs().max(1.);
    let log_error = relative_error.log10();
    println!("{}", log_error);
    log_error < -(precision as f64)
}

/// Similar to `float_eq`, but for complex numbers. Relative error is computed
/// based on the magnitude of `b`.
pub fn complex_eq(a: Complex, b: impl Into<Complex>, precision: u32) -> bool {
    let b = b.into();
    let (a_re, a_im) = (a.real as f64, a.imag as f64);
    let (b_re, b_im) = (b.real as f64, b.imag as f64);
    let relative_error = (((a_re - b_re).powi(2) + (a_im - b_im).powi(2))
        / (b_re.powi(2) + b_im.powi(2)).max(1.))
    .sqrt();
    let log_error = relative_error.log10();
    log_error < -(precision as f64)
}

/// Converts an array of pairs (Num, Num) into an array of complex numbers.
pub fn complex_vec<T: Into<Complex> + Clone>(slice: &[T]) -> Vec<Complex> {
    slice.iter().cloned().map(|p| p.into()).collect()
}

/// Performs a comparison using `complex_eq` for each element in the arrays.
pub fn complex_slice_eq(a: &[Complex], b: &[Complex], precision: u32) -> bool {
    assert!(a.len() == b.len());
    a.iter()
        .zip(b.iter())
        .all(|(&ac, &bc)| complex_eq(ac, bc, precision))
}

/// Performs a comparison using `float_eq` for each element in the arrays.
pub fn float_slice_eq(a: &[Num], b: &[Num], precision: u32) -> bool {
    assert!(a.len() == b.len());
    a.iter()
        .zip(b.iter())
        .all(|(&ac, &bc)| float_eq(ac, bc, precision))
}
