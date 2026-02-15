// tests/longdiv_limbs.rs

#[cfg(test)]
mod test_longdiv_limbs {
    use marint::MarInt; // <-- change this if needed
    use std::cmp::Ordering;

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
        match limbs.len() {
            0 => 0,
            1 => limbs[0] as u128,
            _ => (limbs[0] as u128) | ((limbs[1] as u128) << 64),
        }
    }

    /// For LE normalized limbs: non-empty and no MS zero limbs (unless exactly [0]).
    fn assert_normalized_le(limbs: &[u64]) {
        assert!(!limbs.is_empty(), "limbs must not be empty");
        if limbs.len() > 1 {
            assert_ne!(
                *limbs.last().unwrap(),
                0,
                "normalized limbs must not have MS zeros"
            );
        }
    }

    /// Checks r < b (unsigned magnitude compare).
    fn assert_rem_lt_div(rem: &[u64], div: &[u64]) {
        let ord = MarInt::cmp_limbs(rem, div);
        assert!(
            ord == Ordering::Less,
            "remainder must be < divisor; got rem={rem:?}, div={div:?}"
        );
    }

    /// Make an "unnormalized" variant by appending MS zeros (LE => push zeros).
    fn with_ms_zeros(mut limbs: Vec<u64>, k: usize) -> Vec<u64> {
        for _ in 0..k {
            limbs.push(0);
        }
        limbs
    }

    #[test]
    fn longdiv_basic_cases_u128_domain() {
        let cases: &[(u128, u128)] = &[
            (0, 1),
            (1, 1),
            (2, 1),
            (10, 3),
            (10, 7),
            (u64::MAX as u128, 7),
            ((1u128 << 64) + 5, 2),
            ((1u128 << 64) + 5, 3),
            ((1u128 << 80) + 12345, 97),
            ((1u128 << 127) - 1, (1u128 << 63) + 1),
            ((1u128 << 127) - 1, (1u128 << 64) - 59),
        ];

        for &(a_u, b_u) in cases {
            assert!(b_u != 0);

            let a = limbs_from_u128(a_u);
            let b = limbs_from_u128(b_u);

            let (q, r) = MarInt::longdiv_limbs(&a, &b);

            assert_normalized_le(&q);
            assert_normalized_le(&r);

            // r < b unless b == 0 (not allowed)
            if !(r.len() == 1 && r[0] == 0) {
                assert_rem_lt_div(&r, &b);
            } else {
                // r == 0 is always < b for b>0
                assert!(b_u > 0);
            }

            // Verify exact arithmetic in u128 domain
            let q_u = u128_from_limbs(&q);
            let r_u = u128_from_limbs(&r);

            assert_eq!(q_u, a_u / b_u, "quotient mismatch for a={a_u} b={b_u}");
            assert_eq!(r_u, a_u % b_u, "remainder mismatch for a={a_u} b={b_u}");
            assert_eq!(q_u * b_u + r_u, a_u, "recomposition failed");
        }
    }

    #[test]
    fn longdiv_a_less_than_b_returns_zero_and_a() {
        let a = vec![5u64];
        let b = vec![7u64];

        let (q, r) = MarInt::longdiv_limbs(&a, &b);

        assert_eq!(q, MarInt::limbs_zero());
        assert_eq!(r, a);
    }

    #[test]
    fn longdiv_a_equal_b_returns_one_and_zero() {
        let a = vec![0xDEAD_BEEF_u64, 0x1234_5678_u64];
        let b = a.clone();

        let (q, r) = MarInt::longdiv_limbs(&a, &b);

        // If you renamed to limbs_one(), adjust accordingly.
        assert_eq!(q, MarInt::limbs_one());
        assert_eq!(r, MarInt::limbs_zero());
    }

    #[test]
    fn longdiv_divisor_one_identity() {
        let a = vec![11u64, 22u64, 0, 0]; // deliberately unnormalized
        let b = vec![1u64];

        let (q, r) = MarInt::longdiv_limbs(&a, &b);

        assert_normalized_le(&q);
        assert_eq!(r, MarInt::limbs_zero());

        // Compare against normalized(a)
        let mut expected = a.clone();
        MarInt::normalize_limbs(&mut expected);
        assert_eq!(q, expected);
    }

    #[test]
    fn longdiv_powers_of_two_divisors_u128_domain() {
        // Great for catching off-by-one in quotient digit estimation.
        let a_vals: &[u128] = &[
            0,
            1,
            2,
            3,
            (1u128 << 64) - 1,
            (1u128 << 64),
            (1u128 << 64) + 123,
            (1u128 << 127) - 1,
        ];

        let b_vals: &[u128] = &[1, 2, 4, 8, 1u128 << 16, 1u128 << 32, 1u128 << 63];

        for &a_u in a_vals {
            for &b_u in b_vals {
                let a = limbs_from_u128(a_u);
                let b = limbs_from_u128(b_u);
                let (q, r) = MarInt::longdiv_limbs(&a, &b);

                assert_normalized_le(&q);
                assert_normalized_le(&r);

                let q_u = u128_from_limbs(&q);
                let r_u = u128_from_limbs(&r);

                assert_eq!(q_u, a_u / b_u);
                assert_eq!(r_u, a_u % b_u);
            }
        }
    }

    #[test]
    fn longdiv_invariance_under_input_ms_zero_padding_u128_domain() {
        // If your longdiv expects normalized inputs only, this test will reveal it.
        // If it fails and you decide "inputs must be normalized", you can remove/adjust it.
        let mut seed: u128 = 0xA5A5_5A5A_F0F0_0F0F_1234_5678_9ABC_DEF0;

        for _ in 0..300 {
            seed = seed
                .wrapping_mul(6364136223846793005u128)
                .wrapping_add(1442695040888963407u128);

            let a_u = seed;
            let mut b_u = (seed >> 1) | 1; // make odd
            if b_u == 0 {
                b_u = 3;
            }

            let a0 = limbs_from_u128(a_u);
            let b0 = limbs_from_u128(b_u);

            // padded versions (unnormalized)
            let mut a1 = with_ms_zeros(a0.clone(), ((seed & 3) as usize) + 1);
            let mut b1 = with_ms_zeros(b0.clone(), (((seed >> 2) & 3) as usize) + 1);
            MarInt::normalize_limbs(&mut a1);
            MarInt::normalize_limbs(&mut b1);

            let (q0, r0) = MarInt::longdiv_limbs(&a0, &b0);
            let (q1, r1) = MarInt::longdiv_limbs(&a1, &b1);

            assert_eq!(u128_from_limbs(&q0), u128_from_limbs(&q1));
            assert_eq!(u128_from_limbs(&r0), u128_from_limbs(&r1));
        }
    }

    #[test]
    fn longdiv_stress_random_u128_domain() {
        // Deterministic “stress”: 2,000 cases, all verifiable against u128.
        let mut seed: u128 = 0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210;

        for _ in 0..2000 {
            seed = seed
                .wrapping_mul(2862933555777941757u128)
                .wrapping_add(3037000493u128);

            let a_u = seed;
            let mut b_u = (seed >> 64) as u128;
            b_u |= 1; // ensure odd
            b_u += 2; // ensure >= 3
            if b_u == 0 {
                b_u = 3;
            }

            let a = limbs_from_u128(a_u);
            let b = limbs_from_u128(b_u);

            let (q, r) = MarInt::longdiv_limbs(&a, &b);

            assert_normalized_le(&q);
            assert_normalized_le(&r);

            // remainder < divisor
            if !(r.len() == 1 && r[0] == 0) {
                assert_rem_lt_div(&r, &b);
            }

            let q_u = u128_from_limbs(&q);
            let r_u = u128_from_limbs(&r);

            // exact oracle
            assert_eq!(q_u, a_u / b_u);
            assert_eq!(r_u, a_u % b_u);
            assert_eq!(q_u * b_u + r_u, a_u);
        }
    }

    // Optional: only include if your longdiv panics on divisor==0.
    // If you instead return Result, adapt accordingly.
    #[test]
    #[should_panic]
    fn longdiv_divisor_zero_panics() {
        let a = vec![5u64];
        let b = vec![0u64];
        let _ = MarInt::longdiv_limbs(&a, &b);
    }
}
