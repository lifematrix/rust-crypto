use marnd::{MPBuilder, MPCfg, MPRng, MPRngExt};

fn seeded_lcg64(seed: &str) -> Box<dyn MPRng> {
    let mut cfg = MPCfg::new();
    cfg.insert("seed", seed);
    MPBuilder::build("lcg64", Some(cfg)).expect("build lcg64")
}

#[test]
fn choice_idx_always_first() {
    let mut rng = seeded_lcg64("1");
    let probs = [1.0, 0.0, 0.0];
    for _ in 0..8 {
        assert_eq!(rng.choice_idx(&probs), 0);
    }
}

#[test]
fn choice_idx_always_last() {
    let mut rng = seeded_lcg64("1");
    let probs = [0.0, 0.0, 1.0];
    for _ in 0..8 {
        assert_eq!(rng.choice_idx(&probs), 2);
    }
}

#[test]
fn choice_returns_element() {
    let mut rng = seeded_lcg64("1");
    let elements = ["a", "b", "c"];
    let probs = [0.0, 1.0, 0.0];
    assert_eq!(rng.choice(&elements, &probs), &"b");
}

#[test]
#[should_panic(expected = "elements and probs must have same length")]
fn choice_rejects_len_mismatch() {
    let mut rng = seeded_lcg64("1");
    let elements = ["a", "b"];
    let probs = [0.5, 0.25, 0.25];
    let _ = rng.choice(&elements, &probs);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Invalid probability at index 1")]
fn choice_idx_rejects_negative() {
    let mut rng = seeded_lcg64("1");
    let probs = [0.5, -0.5, 1.0];
    let _ = rng.choice_idx(&probs);
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "sum of probabilities must be 1.0")]
fn choice_idx_rejects_bad_sum() {
    let mut rng = seeded_lcg64("1");
    let probs = [0.2, 0.2, 0.2];
    let _ = rng.choice_idx(&probs);
}
