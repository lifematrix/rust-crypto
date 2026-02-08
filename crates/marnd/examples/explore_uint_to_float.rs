// #![feature(f128)]

use marnd::{MPCfg, MPBuilder, MPRng};

fn main() {
    let mut cfg = MPCfg::new();
    // cfg.insert("seed", "123");
    // cfg.insert("a", "6364136223846793005");
    // cfg.insert("c", "1442695040888963407");

    let mut rng = MPBuilder::build("lcg64", Some(cfg)).expect("build lcg64");

    {
        println!("{:=<80}", "");
        let s = 1u64 << 53;
        let s1 = 1.0 / (s as f64);
        let s2 = (1.0 / s1) as u64;
        let eq = s == s2;
        println!("s={s} s1={s1} s2={s2} {eq}");

        let x = u64::MAX;
        // ffffffffffffffff
        println!("max of u64 = {x:x}");
        for i in 0..64 {
            let x1 = x >> i; 
            let y1 = x1 as f64;
            let y2 = y1 as u64;
            let eq = x1 == y2;
            println!("i={i} x1={x1:x} y1={y1:.12} y2={y2:x} {eq}");
        }
    }

    {
        println!("{}", "=".repeat(80));
        let s = 1u32 << 24;
        let s1 = 1.0 / (s as f32);
        let s2 = (1.0 / s1) as u32;
        let eq = s == s2;
        println!("s={s} s1={s1} s2={s2} {eq}");

        let x = u32::MAX;
        // ffffffff
        println!("max of u32 = {x:x}");
        for i in 0..32 {
            let x1 = x >> i; 
            let y1 = x1 as f32;
            let y2 = y1 as u32;
            let eq = x1 == y2;
            println!("i={i} x1={x1:x} y1={y1:.12} y2={y2:x} {eq}");
        }
    }

    // {
    //     let x = u128::MAX;
    //     // ffffffff
    //     println!("max of u128 = {x:x}");
    //     for i in 0..128 {
    //         let x1 = x >> i; 
    //         let y1 = x1 as f128;
    //         let y2 = y1 as u128;
    //         let eq = x1 == y2;
    //         println!("i={i} x1={x1:x} y1={y1:?} y2={y2:x} {eq}");
    //     }
    // }
}