use super::util::*;
use crate::correlation_match::CorrelationMatch;

use rand::prelude::*;
use rand::rngs::SmallRng;

#[test]
fn simple_correlation_match() {
    let mut matcher = CorrelationMatch::new(8);
    let a = [1., 1.4, 1., 1., 1.5, 0., 1., -1.];
    let b = [1., 2.];
    let w = [1., 1.];
    let offset = matcher.compute(&a, &b, &w);
    let correct = 3.;
    assert!(float_eq(offset, correct, 1));
}

#[test]
fn random_correlation_match() {
    const N: usize = 1024;
    const M: usize = N/4;

    let mut rng = SmallRng::seed_from_u64(42);
    let mut a = vec![0.; N];
    rng.fill(&mut a[..]);

    let mut matcher = CorrelationMatch::new(N);
    let pos = rng.gen_range(1..N-M);
    let w = vec![1.; M];
    let offset = matcher.compute(&a, &a[pos..][..M], &w);
    assert!(float_eq(offset, pos as f32, 1));
}
