//! Finds the approximate minimum point of a function given three fixed points,
//! using a parabola.
//!
//! Formulas derived in `/dokumentaatio/parabolic_interpolation_formulas.py`

use crate::math::*;

/// Interpolate a function given three points, `(0, a), (1, b), (2, c)`, and
/// find the approximate minimum point. Returns a pair `(x, y)`, describing the minimum point.
/// 
/// For reasonable results, it should hold that 'b <= a && b <= c`.
#[inline]
pub fn get_minimum_point(a: Num, b: Num, c: Num) -> (Num, Num) {
    let v = 3. * a - 4. * b + c;
    let w = 2. * (a - 2. * b + c);
    let position = v / w;
    let value = a - v * position / 4.;
    (position, value)
}
