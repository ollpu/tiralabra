use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, PlotConfiguration, AxisScale};

use tiralabra::Fft;
use tiralabra::math::*;

use rand::prelude::*;
use rand::rngs::SmallRng;
use std::vec::Vec;

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fft");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    let mut rng = SmallRng::seed_from_u64(12);
    for size in (6..=14).map(|e| 1 << e) {
        let prepared = Fft::new(size);
        let initial: Vec<Complex> = (0..size).map(|_| (rng.gen(), rng.gen()).into()).collect();
        let mut buffer = initial.clone();
        group.bench_with_input(BenchmarkId::new("copy and fft", size), &size, |b, _| {
            b.iter(|| {
                buffer[..].copy_from_slice(&initial);
                prepared.fft(black_box(&mut buffer))
            });
        });
        buffer[..].copy_from_slice(&initial);
        group.bench_with_input(BenchmarkId::new("fft and ifft", size), &size, |b, _| {
            b.iter(|| {
                prepared.fft(black_box(&mut buffer));
                prepared.ifft(black_box(&mut buffer))
            });
        });
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
