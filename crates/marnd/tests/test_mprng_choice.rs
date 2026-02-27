use marnd::{MPCfg, MPRng};

fn seeded_lcg64(seed: u64) -> MPRng {
    let mut cfg = MPCfg::new();
    cfg.insert("schema", "Lcg64");
    cfg.insert("a", "2");
    cfg.insert("c", "3");
    cfg.insert("seed", &seed.to_string());
    MPRng::build(&cfg).expect("build should succeed")
}

#[test]
fn choice_idx_empty_probs_returns_zero() {
    let mut rng = seeded_lcg64(1);
    assert_eq!(rng.choice_idx(&[]), 0);
}

#[test]
fn choice_idx_always_first_for_one_hot_first() {
    let mut rng = seeded_lcg64(1);
    let probs = [1.0, 0.0, 0.0];
    for _ in 0..8 {
        assert_eq!(rng.choice_idx(&probs), 0);
    }
}

#[test]
fn choice_idx_always_last_for_one_hot_last() {
    let mut rng = seeded_lcg64(1);
    let probs = [0.0, 0.0, 1.0];
    for _ in 0..8 {
        assert_eq!(rng.choice_idx(&probs), 2);
    }
}

#[test]
fn choice_returns_expected_element_for_one_hot_middle() {
    let mut rng = seeded_lcg64(1);
    let elements = ["a", "b", "c"];
    let probs = [0.0, 1.0, 0.0];
    assert_eq!(rng.choice(&elements, &probs), &"b");
}

#[test]
#[should_panic(expected = "must have same length")]
fn choice_panics_on_len_mismatch() {
    let mut rng = seeded_lcg64(1);
    let elements = ["a", "b"];
    let probs = [0.5, 0.25, 0.25];
    let _ = rng.choice(&elements, &probs);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Invalid probability at index 1")]
fn choice_idx_panics_on_negative_prob_in_debug() {
    let mut rng = seeded_lcg64(1);
    let probs = [0.5, -0.5, 1.0];
    let _ = rng.choice_idx(&probs);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "sum of probabilities must be 1.0")]
fn choice_idx_panics_on_bad_sum_in_debug() {
    let mut rng = seeded_lcg64(1);
    let probs = [0.2, 0.2, 0.2];
    let _ = rng.choice_idx(&probs);
}
