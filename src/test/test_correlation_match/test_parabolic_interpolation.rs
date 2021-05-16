use crate::correlation_match::parabolic_interpolation::parabolic_interpolation_minimum;
use crate::test::util::*;

#[test]
fn simple_parabola_minimum() {
    let (x, y) = parabolic_interpolation_minimum(1., 0., 1.).unwrap();
    assert!(float_eq(x, 1., 6));
    assert!(float_eq(y, 0., 6));
}

#[test]
fn random_parabola_minimum() {
    let (x, y) = parabolic_interpolation_minimum(0.87382, 0.64836, 1.53785).unwrap();
    assert!(float_eq(x, 0.70221, 5));
    assert!(float_eq(y, 0.59893, 5));
}

#[test]
fn parabola_minimum_degenerate() {
    assert!(parabolic_interpolation_minimum(1., 2., 3.).is_none());
}
