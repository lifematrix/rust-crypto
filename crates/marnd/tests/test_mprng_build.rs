use marnd::{MPCfg, MPRng, MRndErr};

#[test]
fn build_lcg64_with_explicit_a_c_seed_succeeds() {
    let mut cfg = MPCfg::new();
    cfg.insert("schema", "Lcg64");
    cfg.insert("a", "2");
    cfg.insert("c", "3");
    cfg.insert("seed", "4");

    let mut rng = MPRng::build(&cfg).expect("build should succeed");
    assert_eq!(rng.next_u64(), 11);
}

#[test]
fn build_lcg64_with_preset_and_seed_succeeds() {
    let mut cfg = MPCfg::new();
    cfg.insert("schema", "Lcg64::DK");
    cfg.insert("seed", "1");

    let mut rng = MPRng::build(&cfg).expect("build should succeed");
    let a = 0x5851_f42d_4c95_7f2d_u64;
    let c = 0x875d_d4e5_e439_c627_u64;
    let expected = 1_u64.wrapping_mul(a).wrapping_add(c);
    assert_eq!(rng.next_u64(), expected);
}

#[test]
fn build_unknown_engine_returns_unknown_engine() {
    let mut cfg = MPCfg::new();
    cfg.insert("schema", "BadEngine");
    cfg.insert("seed", "1");

    let err = MPRng::build(&cfg).expect_err("build should fail");
    match err {
        MRndErr::UnknownEngine(_) => {}
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn build_unknown_preset_returns_unknown_preset() {
    let mut cfg = MPCfg::new();
    cfg.insert("schema", "Lcg64::NOPE");
    cfg.insert("seed", "1");

    let err = MPRng::build(&cfg).expect_err("build should fail");
    match err {
        MRndErr::UnknownPreset {
            engine,
            preset,
            available,
        } => {
            assert_eq!(engine, "Lcg64");
            assert_eq!(preset, "NOPE");
            assert!(available.contains(&"DK"));
            assert!(available.contains(&"SV"));
            assert!(available.contains(&"PCG64"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn build_missing_schema_returns_bad_cfg() {
    let cfg = MPCfg::new();
    let err = MPRng::build(&cfg).expect_err("build should fail");
    match err {
        MRndErr::BadCfg(_) => {}
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn build_missing_a_or_c_without_preset_returns_bad_cfg() {
    let mut cfg_missing_a = MPCfg::new();
    cfg_missing_a.insert("schema", "Lcg64");
    cfg_missing_a.insert("c", "3");
    cfg_missing_a.insert("seed", "1");
    let err = MPRng::build(&cfg_missing_a).expect_err("build should fail");
    match err {
        MRndErr::BadCfg(_) => {}
        other => panic!("unexpected error for missing a: {other:?}"),
    }

    let mut cfg_missing_c = MPCfg::new();
    cfg_missing_c.insert("schema", "Lcg64");
    cfg_missing_c.insert("a", "2");
    cfg_missing_c.insert("seed", "1");
    let err = MPRng::build(&cfg_missing_c).expect_err("build should fail");
    match err {
        MRndErr::BadCfg(_) => {}
        other => panic!("unexpected error for missing c: {other:?}"),
    }
}
