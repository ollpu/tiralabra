use crate::math::*;

const EPS: Num = 1e-8;

/// Finds the approximate minimum point of a function given three fixed points,
/// using a parabola.
///
/// Formulas derived in `/dokumentaatio/parabolic_interpolation_formulas.py`
///
/// Given three points, `(0, a), (1, b), (2, c)`, finds the approximate minimum point. Returns a
/// pair `(x, y)`, describing that point. May also return None, if there is no minimum point or it
/// is not on the interval [0, 2].
#[inline]
pub fn parabolic_interpolation_minimum(a: Num, b: Num, c: Num) -> Option<(Num, Num)> {
    // x^2 coefficient should be positive: parabola opens upwards
    let x2coefficient = 2. * (a - 2. * b + c);
    if x2coefficient > EPS {
        let v = 3. * a - 4. * b + c;
        let position = v / x2coefficient;
        if (0. ..=2.).contains(&position) {
            let value = a - v * position / 4.;
            Some((position, value))
        } else {
            None
        }
    } else {
        None
    }
}
