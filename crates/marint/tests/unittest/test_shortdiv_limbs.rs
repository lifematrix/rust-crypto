#[cfg(test)]
mod test_shortdiv_limbs {
    use marint::MSgn;
    use marint::MSgn::*;
    use marint::MarInt;

    fn limbs_from_u128(x: u128) -> Vec<u64> {
        let lo = (x & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        let hi = (x >> 64) as u64;
        if hi == 0 {
            vec![lo]
        } else {
            vec![lo, hi] // little-endian
        }
    }

    fn u128_from_limbs(limbs: &[u64]) -> u128 {
        // Only valid for up to 2 limbs; fine for these tests.
        match limbs.len() {
            0 => 0,
            1 => limbs[0] as u128,
            _ => (limbs[0] as u128) | ((limbs[1] as u128) << 64),
        }
    }

    fn assert_normalized_le(limbs: &[u64]) {
        assert!(!limbs.is_empty(), "normalized limbs must not be empty");
        assert!(
            limbs.len() == 1 || *limbs.last().unwrap() != 0,
            "no MS zero limbs"
        );
    }

    #[test]
    fn shortdiv_by_u64_zero_input() {
        let a = MarInt::limbs_zero();
        let (q, r) = MarInt::shortdiv_limbs_by_u64(&a, 7);
        assert_eq!(q, MarInt::limbs_zero());
        assert_eq!(r, 0);
        assert_normalized_le(&q);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn shortdiv_by_u64_divisor_zero_panics() {
        let a = vec![5u64];
        let _ = MarInt::shortdiv_limbs_by_u64(&a, 0);
    }

    #[test]
    fn shortdiv_by_u64_divisor_one_identity() {
        let a = vec![123u64, 456u64, 0u64, 0u64]; // deliberately has MS zeros
        let (q, r) = MarInt::shortdiv_limbs_by_u64(&a, 1);
        assert_eq!(r, 0);
        assert_normalized_le(&q);
        // should equal a normalized copy
        let mut expected = a.clone();
        MarInt::normalize_limbs(&mut expected);
        assert_eq!(q, expected);
    }

    #[test]
    fn shortdiv_small_values_match_u128() {
        let cases: &[(u128, u64)] = &[
            (0, 3),
            (1, 3),
            (2, 3),
            (10, 3),
            (u64::MAX as u128, 7),
            ((1u128 << 64) + 5, 2),
            ((1u128 << 80) + 12345, 97),
            ((1u128 << 127) - 1, u64::MAX),
        ];

        for &(x, d) in cases {
            let a = limbs_from_u128(x);
            let (q, r) = MarInt::shortdiv_limbs_by_u64(&a, d);

            assert_normalized_le(&q);
            assert!(r < d, "remainder must be < divisor");

            let q_u128 = u128_from_limbs(&q);
            let expected_q = x / (d as u128);
            let expected_r = (x % (d as u128)) as u64;

            assert_eq!(q_u128, expected_q, "quotient mismatch for x={x} d={d}");
            assert_eq!(r, expected_r, "remainder mismatch for x={x} d={d}");
        }
    }

    #[test]
    fn shortdiv_reconstruction_check_u128_domain() {
        // Deterministic pseudo-random test over values that fit into u128 (<=2 limbs).
        // This verifies: a == q*d + r and r < d.
        let mut seed: u128 = 0x1234_5678_9ABC_DEF0_1111_2222_3333_4444;

        for _ in 0..500 {
            // simple LCG
            seed = seed.wrapping_mul(6364136223846793005u128).wrapping_add(1);

            let x = seed; // u128
            let d = ((seed >> 64) as u64).wrapping_add(2); // ensure >=2, never 0/1

            let a = limbs_from_u128(x);
            let (q, r) = MarInt::shortdiv_limbs_by_u64(&a, d);

            assert_normalized_le(&q);
            assert!(r < d);

            let q_u128 = u128_from_limbs(&q);
            let recomposed = q_u128.wrapping_mul(d as u128).wrapping_add(r as u128);

            assert_eq!(recomposed, x, "recompose failed for x={x} d={d}");
        }
    }
}
