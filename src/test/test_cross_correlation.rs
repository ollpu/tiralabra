use super::util::*;
use crate::cross_correlation::CrossCorrelation;

#[test]
fn simple_cross_correlation() {
    let mut cc = CrossCorrelation::new(8);
    let a = [0., 1., 0., 1., 0., 0., 1., 0.];
    let b = [1., 0., 2.];
    let result: Vec<_> = cc.compute(&a, &b).collect();
    let correct = [0., 2., 0., 3., 0., 1., 2., 0., 1., 0.];
    assert!(float_slice_eq(&result, &correct, 6));
}

#[test]
fn simple_cross_correlation_truncated() {
    let mut cc = CrossCorrelation::new(8);
    let a = [0., 1., 0., 1., 0., 0., 1., 0.];
    let b = [1., 0., 2.];
    let result: Vec<_> = cc.compute_truncated(&a, &b).collect();
    // Same as in `simple_cross_correlation`, but two elements are omitted from
    // the beginning and end. These correspond to the partially overlapping positions.
    let correct = [0., 3., 0., 1., 2., 0.];
    assert!(float_slice_eq(&result, &correct, 6));
}

#[test]
fn cross_correlation_both_max_size() {
    let mut cc = CrossCorrelation::new(4);
    let a = [1., 1., 0., 1.];
    let b = [1., 0., 0., -1.];
    let result: Vec<_> = cc.compute(&a, &b).collect();
    let correct = [-1., -1., 0., 0., 1., 0., 1.];
    assert!(float_slice_eq(&result, &correct, 6));
}
