use marnd::{MPCfg, MPRng};

fn main() {
    let mut cfg = MPCfg::new();
    cfg.insert("schema", "MLcg64");
    cfg.insert("seed", "1");
    cfg.insert("a", "0x5851_f42d_4c95_7f2d");
    cfg.insert("c", "0x875d_d4e5_e439_c627");

    let mut rng = MPRng::build(&cfg).expect(format!("Failed to build MPRng (Pseudorandom Number Generator) from configuration {:?}.", cfg).as_str());
    println!("rng: {}", rng);
    let a = 0x5851_f42d_4c95_7f2d_u64;
    let c = 0x875d_d4e5_e439_c627_u64;
    let expected = 1_u64.wrapping_mul(a).wrapping_add(c);
    assert_eq!(rng.next_u64(), expected);

    // for _ in 0..100 {
    //     println!("{:x}", rng.next_u64());
    // }

    // for _ in 0..100 {
    //     println!("{:.10}", rng.next_f64());
    // }
}
