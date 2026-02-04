use marnd::{MBErr, MPBuilder, MPCfg, MPRng};

#[test]
fn build_lcg64_with_seed_uses_defaults() {
    let mut cfg = MPCfg::new();
    cfg.insert("seed", "1");

    let mut rng = MPBuilder::build("lcg64", Some(cfg)).expect("build lcg64");
    let expected = 1u64
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    assert_eq!(rng.next_u64(), expected);
}

#[test]
fn build_lcg64_with_hex_params() {
    let mut cfg = MPCfg::new();
    cfg.insert("a", "0x2");
    cfg.insert("c", "0x3");
    cfg.insert("seed", "0x4");

    let mut rng = MPBuilder::build("lcg64", Some(cfg)).expect("build lcg64");
    assert_eq!(rng.next_u64(), 11);
}

#[test]
fn mpcfg_get_u64_or_default() {
    let cfg = MPCfg::new();
    let v = cfg.get_u64_or("a", 7).expect("get_u64_or");
    assert_eq!(v, 7);
}

#[test]
fn mpcfg_get_u64_or_invalid() {
    let mut cfg = MPCfg::new();
    cfg.insert("a", "nope");

    let err = cfg.get_u64_or("a", 7).expect_err("expected parse error");
    match err {
        MBErr::BadCfg(_) => {}
        other => panic!("unexpected error: {other:?}"),
    }
}
