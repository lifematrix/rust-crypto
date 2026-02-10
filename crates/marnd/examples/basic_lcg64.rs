use marnd::{MPCfg, MPBuilder, MPRng};

fn main() {
    println!("infinity: {}, {}", f32::INFINITY== f32::INFINITY, f32::NEG_INFINITY==f32::NEG_INFINITY);
    let mut cfg = MPCfg::new();
    // cfg.insert("seed", "123");
    // cfg.insert("a", "6364136223846793005");
    // cfg.insert("c", "1442695040888963407");

    let mut rng = MPBuilder::build("lcg64", Some(cfg)).expect("build lcg64");

    let r64 = rng.next_u64();
    let r32 = rng.next_u32();
    let rf64 = rng.next_f64();
    let rf32 = rng.next_f32();
    let rb = rng.next_bool();

    println!("u64={r64} u32={r32} f64={rf64} f32={rf32} bool={rb}");

    for _ in 0..100 {
        println!("{}", rng.next_u64() >> 60); 
    }
}
