use crate::Float;

/// Finds the approximate minimum point of a function given three fixed points,
/// using a parabola.
///
/// Formulas derived in `/misc/parabolic_interpolation_formulas.py`
///
/// Given three points, `(0, a), (1, b), (2, c)`, finds the approximate minimum point. Returns a
/// pair `(x, y)`, describing that point. May also return None, if there is no minimum point or it
/// is not on the interval [0, 2].
#[inline]
pub fn parabolic_interpolation_minimum<Num: Float>(a: Num, b: Num, c: Num) -> Option<(Num, Num)> {
    // x^2 coefficient should be positive: parabola opens upwards
    let eps = Num::v(10.).powi(-8);
    let x2coefficient = Num::v(2.) * (a - Num::v(2.) * b + c);
    if x2coefficient > eps {
        let v = Num::v(3.) * a - Num::v(4.) * b + c;
        let position = v / x2coefficient;
        if (Num::v(0.)..=Num::v(2.)).contains(&position) {
            let value = a - v * position / Num::v(4.);
            Some((position, value))
        } else {
            None
        }
    } else {
        None
    }
}
