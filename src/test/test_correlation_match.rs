use super::util::*;
use crate::correlation_match::CorrelationMatch;

#[test]
fn simle_correlation_match() {
    let mut matcher = CorrelationMatch::new(8);
    let a = [1., 1.4, 1., 1., 1.5, 0., 1., -1.];
    let b = [1., 2.];
    let w = [1., 1.];
    let offset = matcher.compute(&a, &b, &w);
    let correct = 3.;
    panic!();
    assert!(float_eq(offset, correct, 1));
}
