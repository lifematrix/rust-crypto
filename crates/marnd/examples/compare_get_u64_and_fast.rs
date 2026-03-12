use std::hint::black_box;
use std::time::Instant;

// Adjust this import path to match your crate layout.
use marnd::{MPRng, MPCfg};


fn build_rng() -> MPRng {
    let mut cfg = MPCfg::new();
    cfg.insert("schema", "Lcg64::DK");
    cfg.insert("seed", "0x123456789abcdef0");
    MPRng::build(&cfg).expect("MPRng::build should succeed")
}

fn bench_next_f64(mut rng: MPRng, n: usize) -> f64 {
    let start = Instant::now();
    let mut acc = 0.0_f64;

    for _ in 0..n {
        acc += black_box(rng.next_f64());
    }

    let elapsed = start.elapsed();
    let ns_per_call = elapsed.as_secs_f64() * 1e9 / n as f64;

    println!(
        "next_f64      : total = {:?}, ns/call = {:.3}, checksum = {}",
        elapsed, ns_per_call, acc
    );

    acc
}

fn bench_next_f64_fast(mut rng: MPRng, n: usize) -> f64 {
    let start = Instant::now();
    let mut acc = 0.0_f64;

    for _ in 0..n {
        acc += black_box(rng.next_f64_fast());
    }

    let elapsed = start.elapsed();
    let ns_per_call = elapsed.as_secs_f64() * 1e9 / n as f64;

    println!(
        "next_f64_fast : total = {:?}, ns/call = {:.3}, checksum = {}",
        elapsed, ns_per_call, acc
    );

    acc
}

fn main() {
    let n = 1_000_000_000;
    // let n = 1_000_000;

    // Adjust construction to your actual API.
    let rng1 = build_rng();
    let rng2 = build_rng();

    let _ = bench_next_f64(rng1, n);
    let _ = bench_next_f64_fast(rng2, n);
}