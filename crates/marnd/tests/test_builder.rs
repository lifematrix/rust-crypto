use marnd::{MBErr, MPBuilder, MPCfg};

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
fn build_lcg64_variant_a_with_seed_uses_a_constants() {
    let mut cfg = MPCfg::new();
    cfg.insert("seed", "1");

    let mut rng = MPBuilder::build("Lcg64::A", Some(cfg)).expect("build lcg64::a");
    let expected = 1u64
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    assert_eq!(rng.next_u64(), expected);
}

#[test]
fn build_lcg64_variant_b_with_seed_uses_b_constants() {
    let mut cfg = MPCfg::new();
    cfg.insert("seed", "1");

    let mut rng = MPBuilder::build("Lcg64::B", Some(cfg)).expect("build lcg64::b");
    let expected = 1u64
        .wrapping_mul(2862933555777941757)
        .wrapping_add(3037000493);
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
fn build_with_unknown_lcg64_variant_errors() {
    match MPBuilder::build("Lcg64::Z", None) {
        Err(MBErr::UnknownVariant { .. }) => {}
        Err(other) => panic!("unexpected error: {other:?}"),
        Ok(_) => panic!("expected unknown variant"),
    }
}

#[test]
fn build_with_invalid_scheme_errors() {
    match MPBuilder::build("Lcg64::A::Extra", None) {
        Err(MBErr::InvalidScheme(_)) => {}
        Err(other) => panic!("unexpected error: {other:?}"),
        Ok(_) => panic!("expected invalid scheme"),
    }
}

#[test]
fn build_family_only_case_insensitive() {
    let mut cfg = MPCfg::new();
    cfg.insert("seed", "1");

    let mut rng = MPBuilder::build("LCG64", Some(cfg)).expect("build lcg64");
    let expected = 1u64
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    assert_eq!(rng.next_u64(), expected);
}

#[test]
fn build_variant_allows_cfg_overrides() {
    let mut cfg = MPCfg::new();
    cfg.insert("seed", "4");
    cfg.insert("a", "2");
    cfg.insert("c", "3");

    let mut rng = MPBuilder::build("Lcg64::B", Some(cfg)).expect("build lcg64::b");
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
