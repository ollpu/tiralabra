use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tiralabra::CorrelationMatch;

use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(12);
    for size in (512..=2048).step_by(256) {
        let half_size = size / 2;
        let mut a = vec![0.; size];
        let w = vec![1.; half_size];
        rng.fill(&mut a[..]);
        let mut matcher = CorrelationMatch::new(size);
        c.bench_with_input(BenchmarkId::new("correlation-match", size), &size, |b, _| {
            b.iter(|| {
                let pos = rng.gen_range(0..size - half_size);
                matcher.compute(black_box(&a), black_box(&a[pos..][..half_size]), &w)
            });
        });
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
