use crate::test::util::*;
use crate::correlation_match::parabolic_interpolation::get_minimum_point;

#[test]
fn simple_parabola_minimum() {
    let (x, y) = get_minimum_point(1., 0., 1.).unwrap();
    assert!(float_eq(x, 1., 6));
    assert!(float_eq(y, 0., 6));
}

#[test]
fn random_parabola_minimum() {
    let (x, y) = get_minimum_point(0.87382, 0.64836, 1.53785).unwrap();
    assert!(float_eq(x, 0.70221, 5));
    assert!(float_eq(y, 0.59893, 5));
}

#[test]
fn parabola_minimum_degenerate() {
    assert!(get_minimum_point(1., 2., 3.).is_none());
}

