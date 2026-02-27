use std::hint::black_box;
use std::time::Instant;

use marnd::{MPCfg, MPRng};

const N_SAMPLES: usize = 100_000_000;

fn build_rng() -> MPRng {
    let mut cfg = MPCfg::new();
    cfg.insert("schema", "Lcg64::DK");
    cfg.insert("seed", "0x123456789abcdef0");
    MPRng::build(&cfg).expect("MPRng::build should succeed")
}

fn bench_u64() {
    let mut rng = build_rng();
    let start = Instant::now();
    let mut acc = 0u64;
    for _ in 0..N_SAMPLES {
        acc ^= rng.next_u64();
    }
    let elapsed = start.elapsed().as_secs_f64();
    let throughput = (N_SAMPLES as f64) / elapsed;
    black_box(acc);

    println!(
        "next_u64: {:>12.2} million ops/s ({N_SAMPLES} samples in {:.3} s)",
        throughput / 1e6,
        elapsed
    );
}

fn bench_f64() {
    let mut rng = build_rng();
    let start = Instant::now();
    let mut acc = 0.0f64;
    for _ in 0..N_SAMPLES {
        acc += rng.next_f64();
    }
    let elapsed = start.elapsed().as_secs_f64();
    let throughput = (N_SAMPLES as f64) / elapsed;
    black_box(acc);

    println!(
        "next_f64: {:>12.2} million ops/s ({N_SAMPLES} samples in {:.3} s)",
        throughput / 1e6,
        elapsed
    );
}

fn main() {
    println!("MPRng throughput benchmark (single-threaded, release mode)");
    bench_u64();
    bench_f64();
}
