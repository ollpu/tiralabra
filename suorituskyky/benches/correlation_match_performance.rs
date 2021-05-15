use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tiralabra::CorrelationMatch;

use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn benchmark(c: &mut Criterion) {
    const N: usize = 1024;
    const M: usize = N / 4;

    let mut rng = SmallRng::seed_from_u64(12);
    let mut a = vec![0.; N];
    let w = vec![1.; M];
    rng.fill(&mut a[..]);
    let mut matcher = CorrelationMatch::new(N);
    c.bench_function("match random 1024", |b| {
        b.iter(|| {
            let pos = rng.gen_range(1..N - M);
            matcher.compute(black_box(&a), black_box(&a[pos..][..M]), &w)
        });
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
