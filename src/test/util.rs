use crate::math::{Complex, Num};

/// Varmistaa, `a` poikkeaa luvusta `b` korkeintaan `precision` merkitsevää
/// desimaalia. `b` on n.s. oikea vastaus, eli merkitsevien desimaalien määrä
/// lasketaan siitä.
pub fn float_eq(a: Num, b: Num, precision: u32) -> bool {
    // Muutetaan luvut ensin tuplatarkkuisiksi, että vertailussa ei synny
    // turhaan enempää virhettä.
    let a = a as f64;
    let b = b as f64;
    let relative_error = (a - b).abs() / b;
    let log_error = relative_error.log10();
    log_error < -(precision as f64)
}

/// Sama kuin `float_eq`, mutta kompleksiluvuille. Suhteellinen virhe lasketaan
/// kompleksiluvun `b` itseisarvon perusteella.
pub fn complex_eq(a: Complex, b: impl Into<Complex>, precision: u32) -> bool {
    let b = b.into();
    let (a_re, a_im) = (a.real as f64, a.imag as f64);
    let (b_re, b_im) = (b.real as f64, b.imag as f64);
    let relative_error =
        (((a_re - b_re).powi(2) + (a_im - b_im).powi(2)) / (b_re.powi(2) + b_im.powi(2))).sqrt();
    let log_error = relative_error.log10();
    log_error < -(precision as f64)
}

/// Muuntaa listan pareista (luku, luku) listaksi kompleksilukuja.
pub fn complex_vec<T: Into<Complex> + Clone>(slice: &[T]) -> Vec<Complex> {
    slice.iter().cloned().map(|p| p.into()).collect()
}

/// Suorittaa vertailun käyttäen `complex_eq` jokaiselle listan alkiolle.
pub fn complex_slice_eq(a: &[Complex], b: &[Complex], precision: u32) -> bool {
    assert!(a.len() == b.len());
    a.iter()
        .zip(b.iter())
        .all(|(&ac, &bc)| complex_eq(ac, bc, precision))
}
