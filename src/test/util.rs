/// Ensures that `a` differs from the number `b` by at most `precision` significant
/// digits. `b` is the "correct answer", so the relative error is computed based on
/// its magnitude. When `b < 1`, absolute error is used instead.
pub fn float_eq(a: f32, b: f32, precision: u32) -> bool {
    // We convert the numbers into double precision first, so that no further
    // errors are introduced while comparing.
    let a = a as f64;
    let b = b as f64;
    let relative_error = (a - b).abs() / b.abs().max(1.);
    let log_error = relative_error.log10();
    log_error < -(precision as f64)
}

/// Performs a comparison using `float_eq` for each element in the arrays.
pub fn float_slice_eq(a: &[f32], b: &[f32], precision: u32) -> bool {
    assert!(a.len() == b.len());
    a.iter()
        .zip(b.iter())
        .all(|(&ac, &bc)| float_eq(ac, bc, precision))
}
