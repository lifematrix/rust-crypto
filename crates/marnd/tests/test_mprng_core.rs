use marnd::{MPCfg, MPRng};

fn build_lcg64(a: u64, c: u64, seed: u64) -> MPRng {
    let mut cfg = MPCfg::new();
    cfg.insert("schema", "Lcg64");
    cfg.insert("a", &a.to_string());
    cfg.insert("c", &c.to_string());
    cfg.insert("seed", &seed.to_string());
    MPRng::build(&cfg).expect("build should succeed")
}

fn lcg_step(state: u64, a: u64, c: u64) -> u64 {
    state.wrapping_mul(a).wrapping_add(c)
}

#[test]
fn next_u64_sequence_matches_lcg_formula() {
    let a = 6364136223846793005_u64;
    let c = 1442695040888963407_u64;
    let seed = 1_u64;
    let mut rng = build_lcg64(a, c, seed);

    let mut state = seed;
    for _ in 0..8 {
        state = lcg_step(state, a, c);
        assert_eq!(rng.next_u64(), state);
    }
}

#[test]
fn next_u32_matches_upper_32_bits_of_next_u64() {
    let a = 2862933555777941757_u64;
    let c = 3037000493_u64;
    let seed = 7_u64;
    let mut rng = build_lcg64(a, c, seed);

    let expected_state = lcg_step(seed, a, c);
    let expected = (expected_state >> 32) as u32;
    assert_eq!(rng.next_u32(), expected);
}

#[test]
fn next_f64_in_half_open_unit_interval() {
    let mut rng = build_lcg64(2, 3, 4);
    for _ in 0..256 {
        let x = rng.next_f64();
        assert!(x >= 0.0);
        assert!(x < 1.0);
    }
}

#[test]
fn next_f32_in_half_open_unit_interval() {
    let mut rng = build_lcg64(2, 3, 4);
    for _ in 0..256 {
        let x = rng.next_f32();
        assert!(x >= 0.0);
        assert!(x < 1.0);
    }
}

#[test]
fn next_bool_produces_boolean_from_msb() {
    let a = 2862933555777941757_u64;
    let c = 3037000493_u64;
    let seed = 9_u64;
    let mut rng = build_lcg64(a, c, seed);

    let expected_state = lcg_step(seed, a, c);
    let expected = (expected_state >> 63) != 0;
    assert_eq!(rng.next_bool(), expected);
}

#[test]
fn fill_writes_little_endian_stream_and_handles_tail() {
    let a = 6364136223846793005_u64;
    let c = 1442695040888963407_u64;
    let seed = 42_u64;

    for size in [0_usize, 1, 7, 8, 9, 17] {
        let mut rng = build_lcg64(a, c, seed);
        let mut got = vec![0_u8; size];
        rng.fill(&mut got);

        let words = size.div_ceil(8);
        let mut expected_bytes = Vec::with_capacity(words * 8);
        let mut state = seed;
        for _ in 0..words {
            state = lcg_step(state, a, c);
            expected_bytes.extend_from_slice(&state.to_le_bytes());
        }
        let expected = &expected_bytes[..size];
        assert_eq!(got, expected);
    }
}

#[test]
fn seed_resets_sequence() {
    let a = 6364136223846793005_u64;
    let c = 1442695040888963407_u64;
    let seed = 123_u64;
    let mut rng = build_lcg64(a, c, seed);

    let first = rng.next_u64();
    let second = rng.next_u64();
    assert_ne!(first, second);

    rng.seed(seed);
    assert_eq!(rng.next_u64(), first);
    assert_eq!(rng.next_u64(), second);
}
