use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use marnd::{Lcg64, MPCfg, MPRng};

fn build_rng() -> MPRng {
    let mut cfg = MPCfg::new();
    cfg.insert("schema", "Lcg64::DK");
    cfg.insert("seed", "0x123456789abcdef0");
    MPRng::build(&cfg).expect("MPRng::build should succeed")
}

fn bench_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("mprng_u64");
    for &batch in &[1_024usize, 16_384, 262_144] {
        group.throughput(Throughput::Elements(batch as u64));
        group.bench_with_input(BenchmarkId::from_parameter(batch), &batch, |b, &n| {
            let mut rng = build_rng();
            b.iter(|| {
                let mut acc = 0u64;
                for _ in 0..n {
                    acc ^= rng.next_u64();
                }
                black_box(acc);
            });
        });
    }
    group.finish();
}

fn bench_lcg64_u64_direct(c: &mut Criterion) {
    let mut group = c.benchmark_group("lcg64_u64_direct");
    for &batch in &[1_024usize, 16_384, 262_144] {
        group.throughput(Throughput::Elements(batch as u64));
        group.bench_with_input(BenchmarkId::from_parameter(batch), &batch, |b, &n| {
            let mut lcg = Lcg64::from_preset("DK", 0x1234_5678_9abc_def0)
                .expect("Lcg64::from_preset should succeed");
            b.iter(|| {
                let mut acc = 0u64;
                for _ in 0..n {
                    acc ^= lcg.next_u64();
                }
                black_box(acc);
            });
        });
    }
    group.finish();
}

fn bench_f64(c: &mut Criterion) {
    let mut group = c.benchmark_group("mprng_f64");
    for &batch in &[1_024usize, 16_384, 262_144] {
        group.throughput(Throughput::Elements(batch as u64));
        group.bench_with_input(BenchmarkId::from_parameter(batch), &batch, |b, &n| {
            let mut rng = build_rng();
            b.iter(|| {
                let mut acc = 0.0f64;
                for _ in 0..n {
                    acc += rng.next_f64();
                }
                black_box(acc);
            });
        });
    }
    group.finish();
}

fn u64_to_unit_f64(x: u64) -> f64 {
    const SCALE: f64 = 1.0 / ((1u64 << 53) as f64);
    let v = x >> 11;
    (v as f64) * SCALE
}

fn bench_u64_to_f64_convert_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("u64_to_f64_convert_only");
    for &batch in &[1_024usize, 16_384, 262_144] {
        let mut src = Vec::with_capacity(batch);
        let mut lcg =
            Lcg64::from_preset("DK", 0x1234_5678_9abc_def0).expect("Lcg64 preset should exist");
        for _ in 0..batch {
            src.push(lcg.next_u64());
        }

        group.throughput(Throughput::Elements(batch as u64));
        group.bench_with_input(BenchmarkId::from_parameter(batch), &batch, |b, &n| {
            b.iter(|| {
                let mut acc = 0.0f64;
                for &x in src.iter().take(n) {
                    acc += u64_to_unit_f64(x);
                }
                black_box(acc);
            });
        });
    }
    group.finish();
}

fn config() -> Criterion {
    Criterion::default().sample_size(30)
}

criterion_group! {
    name = benches;
    config = config();
    targets = bench_u64, bench_lcg64_u64_direct, bench_f64, bench_u64_to_f64_convert_only
}
criterion_main!(benches);
