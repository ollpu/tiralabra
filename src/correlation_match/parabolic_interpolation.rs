//! Finds the approximate minimum point of a function given three fixed points,
//! using a parabola.
//!
//! Formulas derived in `/dokumentaatio/parabolic_interpolation_formulas.py`

use crate::math::*;

const EPS: Num = 1e-8;

/// Interpolate a function given three points, `(0, a), (1, b), (2, c)`, and
/// find the approximate minimum point. Returns a pair `(x, y)`, describing the minimum point.
/// May also return None, if there is no minimum point or it is not on the interval [0, 2].
#[inline]
pub fn get_minimum_point(a: Num, b: Num, c: Num) -> Option<(Num, Num)> {
    let w = 2. * (a - 2. * b + c);
    if w > EPS {
        let v = 3. * a - 4. * b + c;
        let position = v / w;
        if position >= 0. && position <= 2. {
            let value = a - v * position / 4.;
            Some((position, value))
        } else {
            None
        }
    } else {
        None
    }
}
