use std::time::Instant;
use std::hint::black_box;

fn main() {
    let n = 1_000_000_000;
    let mut x = 1u64;
    let mut acc = 0.0;

    let start = Instant::now();
    for _ in 0..n {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        let v = ((x >> 11) as f64) * (1.0 / ((1u64 << 53) as f64));
        // acc += black_box(v);
        acc += v;
    }
    println!("standard kernel {:?}", start.elapsed());
    println!("{}", acc);

    acc = 0.0;
    let start = Instant::now();
    for _ in 0..n {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        let bits = 0x3ff0000000000000u64 | (x >> 12);
        let v = f64::from_bits(bits) - 1.0;
        //acc += black_box(v);
        acc += v;
    }

    println!("fast kernel {:?}", start.elapsed());
    println!("{}", acc);
}