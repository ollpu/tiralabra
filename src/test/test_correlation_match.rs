mod test_parabolic_interpolation;

use crate::correlation_match::CorrelationMatch;

use crate::math::*;
use super::util::*;

use rand::prelude::*;
use rand::rngs::SmallRng;

#[test]
fn simple_correlation_match() {
    let mut matcher = CorrelationMatch::new(8);
    let a = [1., 1.4, 1., 1., 1.9, 0., 1., -1.];
    let b = [1., 2.];
    let w = [1., 1.];
    let (offset, _) = matcher.compute(&a, &b, &w);
    let correct = 3.;
    assert!((offset - correct).abs() < 0.5);
}

#[test]
fn random_correlation_match() {
    const N: usize = 1024;
    const M: usize = N / 4;

    let mut rng = SmallRng::seed_from_u64(42);
    let mut a = vec![0.; N];
    rng.fill(&mut a[..]);

    let mut matcher = CorrelationMatch::new(N);
    let pos = rng.gen_range(1..N - M);
    let w = vec![1.; M];
    let (offset, _) = matcher.compute(&a, &a[pos..][..M], &w);
    assert!((offset - pos as f32).abs() < 0.5);
}

#[test]
fn correlation_match_subsample_accuracy() {
    const N: usize = 32;
    let mut matcher = CorrelationMatch::new(N);
    let pos = 5.315237;
    let a: Vec<_> = (0..N).map(|i| ((i as f32 / N as f32) * 2. * PI).sin()).collect();
    let b: Vec<_> = (0..N/2)
        .map(|i| i as f32 + pos)
        .map(|i| ((i / N as f32) * 2. * PI).sin())
        .collect();
    let w = vec![1.; N/2];
    let (offset, _) = matcher.compute(&a, &b, &w);
    assert!(float_eq(offset, pos, 2));
}

#[test]
fn correlation_match_period() {
    const N: usize = 32;
    const M: usize = 16;
    let mut matcher = CorrelationMatch::new(N);
    let interval = 6.137241;
    let a: Vec<_> = (0..N).map(|i| ((i as f32 / interval) * 2. * PI).sin()).collect();
    let pos = 13;
    let w = vec![1.; M];
    let (_, measured_interval) = matcher.compute(&a, &a[pos..][..M], &w);
    assert!(float_eq(measured_interval.unwrap(), interval, 2));
}
