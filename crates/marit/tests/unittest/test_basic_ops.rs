
#[cfg(test)]
mod tests {
    use marit::sign::MSgn;
    use marit::sign::MSgn::*;
    use marit::marint::MarInt;

    // Helper: check canonical invariant: no leading zeros, except [0].
    fn is_canonical(limbs: &[u64]) -> bool {
        limbs.len() == 1 && limbs[0] == 0 || *limbs.last().unwrap() != 0
    }

    #[test]
    fn test_limbs_add_by_u64_basic() {
        // [0] + u = [u]
        assert_eq!(MarInt::limbs_add_by_u64(&[0], 0), vec![0]);
        assert_eq!(MarInt::limbs_add_by_u64(&[0], 5), vec![5]);

        // no carry
        assert_eq!(MarInt::limbs_add_by_u64(&[7], 3), vec![10]);
        assert_eq!(MarInt::limbs_add_by_u64(&[1, 2], 3), vec![4, 2]);

        // carry within one limb
        assert_eq!(MarInt::limbs_add_by_u64(&[u64::MAX], 1), vec![0, 1]);

        // carry within one limb
        assert_eq!(MarInt::limbs_add_by_u64(&[u64::MAX], 123), vec![122, 1]);

        // carry cascades across multiple limbs
        assert_eq!(
            MarInt::limbs_add_by_u64(&[u64::MAX, u64::MAX], 1),
            vec![0, 0, 1]
        );

        // add to a higher limb via cascade
        assert_eq!(
            MarInt::limbs_add_by_u64(&[u64::MAX, 0, 5], 1),
            vec![0, 1, 5]
        );
    }

    #[test]
    fn test_limbs_add_by_u64_does_not_mutate_input() {
        let input = vec![u64::MAX, 123];
        let _ = MarInt::limbs_add_by_u64(&input, 1);
        assert_eq!(input, vec![u64::MAX, 123]);
    }

    #[test]
    fn test_limbs_add_by_u64_canonical_result() {
        let cases = [
            (&[0][..], 0u64),
            (&[0][..], 9),
            (&[1][..], u64::MAX),
            (&[u64::MAX][..], 1),
            (&[u64::MAX, u64::MAX][..], 1),
            (&[0, 0, 1][..], 5), // even if caller passes non-canonical, result should still be canonical-ish
        ];

        for (limbs, add) in cases {
            let result = MarInt::limbs_add_by_u64(limbs, add);
            assert!(!result.is_empty(), "result must not be empty");
            assert!(is_canonical(&result), "result not canonical: {:?}", result);
        }
    }

    #[test]
    fn test_limbs_add_by_u64_matches_u128_for_small_values() {
        // Property-style test without external crates:
        // For values that fit in u128 (<=2 limbs), compare against u128 arithmetic.
        for a0 in [0u64, 1, 2, 123, u64::MAX - 1, u64::MAX] {
            for a1 in [0u64, 1, 999, u64::MAX] {
                let limbs = [a0, a1];
                // Interpret as u128: a0 + a1*2^64
                let aval = (a0 as u128) + ((a1 as u128) << 64);

                for add in [0u64, 1, 2, 7, 123, u64::MAX] {
                    let got = MarInt::limbs_add_by_u64(&limbs, add);

                    // Convert result (up to 3 limbs possible) back to u128 where possible:
                    // Here, since input is 2 limbs and we add u64, output is at most 3 limbs,
                    // but it still fits in u128 only if the 3rd limb is 0.
                    let mut gval: u128 = 0;
                    if got.len() >= 1 { gval += got[0] as u128; }
                    if got.len() >= 2 { gval += (got[1] as u128) << 64; }
                    if got.len() >= 3 {
                        // if third limb is non-zero, u128 overflow would happen, skip this check
                        if got[2] != 0 { continue; }
                    }

                    let expected = aval + (add as u128);
                    assert_eq!(gval, expected, "limbs={:?}, add={}", limbs, add);
                }
            }
        }
    }

    #[test]
    fn test_limbs_mul_by_u64_zero_multiplier() {
        assert_eq!(MarInt::limbs_mul_by_u64(&[0], 0), vec![0]);
        assert_eq!(MarInt::limbs_mul_by_u64(&[5], 0), vec![0]);
        assert_eq!(MarInt::limbs_mul_by_u64(&[1, 2, 3], 0), vec![0]);
    }

    #[test]
    fn test_limbs_mul_by_u64_one_multiplier() {
        assert_eq!(MarInt::limbs_mul_by_u64(&[0], 1), vec![0]);
        assert_eq!(MarInt::limbs_mul_by_u64(&[7], 1), vec![7]);
        assert_eq!(MarInt::limbs_mul_by_u64(&[7, 8, 9], 1), vec![7, 8, 9]);
    }

    #[test]
    fn test_limbs_mul_by_u64_simple_no_carry() {
        assert_eq!(MarInt::limbs_mul_by_u64(&[3], 5), vec![15]);
        assert_eq!(MarInt::limbs_mul_by_u64(&[1, 2], 2), vec![2, 4]); // (2^64*2 + 1)*2
    }

    #[test]
    fn test_limbs_mul_by_u64_single_limb_with_carry() {
        // (2^64 - 1) * 2 = 2^65 - 2 => limbs: [u64::MAX - 1, 1]
        assert_eq!(
            MarInt::limbs_mul_by_u64(&[u64::MAX], 2),
            vec![u64::MAX - 1, 1]
        );

        // (2^64 - 1) * (2^64 - 1) = 2^128 - 2^65 + 1 => low=1, high=u64::MAX-1
        assert_eq!(
            MarInt::limbs_mul_by_u64(&[u64::MAX], u64::MAX),
            vec![1, u64::MAX - 1]
        );
    }

    #[test]
    fn test_limbs_mul_by_u64_multi_limb_carry_propagation() {
        println!("\x1b[34mDEBUG\x1b[0m Testing is here {}", "test_limbs_mul_by_u64_multi_limb_carry_propagation");

        // [u64::MAX, u64::MAX] represents (2^128 - 1)
        // (2^128 - 1) * 2 = 2^129 - 2 => [u64::MAX - 1, u64::MAX, 1]
        assert_eq!(
            MarInt::limbs_mul_by_u64(&[u64::MAX, u64::MAX], 2),
            vec![u64::MAX - 1, u64::MAX, 1]
        );

        // Multiply with a value that introduces carries in both limbs
        assert_eq!(
            MarInt::limbs_mul_by_u64(&[u64::MAX, 1], 2),
            vec![u64::MAX - 1, 3] // (1*2^64 + (2^64-1)) * 2 = 2*2^64 + (2^65-2) = 3*2^64 + (2^64-2)
        );
    }

    #[test]
    fn test_limbs_mul_by_u64_canonical_result() {
        let cases: [(&[u64], u64); 6] = [
            (&[0], 0),
            (&[0], 123),
            (&[5], 0),
            (&[5], 7),
            (&[u64::MAX], 2),
            (&[1, 0, 0, 0], 9), // even if caller passes non-canonical, result should still trim
        ];

        for (limbs, mul) in cases {
            let result = MarInt::limbs_mul_by_u64(limbs, mul);
            assert!(!result.is_empty(), "result must not be empty");
            assert!(is_canonical(&result), "not canonical: {:?} (mul={})", result, mul);
        }
    }

    #[test]
    fn test_limbs_mul_by_u64_matches_u128_for_small_values() {
        // Compare with u128 arithmetic for values representable by <=2 limbs.
        let limb_vals = [0u64, 1, 2, 7, 123, u64::MAX - 1, u64::MAX];
        let mul_vals = [0u64, 1, 2, 3, 5, 10, u64::MAX];

        for &a0 in &limb_vals {
            for &a1 in &limb_vals {
                let limbs = [a0, a1];
                let aval: u128 = (a0 as u128) + ((a1 as u128) << 64);

                for &m in &mul_vals {
                    let got = MarInt::limbs_mul_by_u64(&limbs, m);

                    // Convert result back into u128 if it fits (i.e. at most 2 limbs)
                    // If a 3rd limb appears and is non-zero, u128 can't represent it; skip.
                    let mut gval: u128 = 0;
                    if got.len() >= 1 { gval += got[0] as u128; }
                    if got.len() >= 2 { gval += (got[1] as u128) << 64; }
                    if got.len() >= 3 {
                        if got[2] != 0 {
                            continue;
                        }
                    }

                    let expected = aval * (m as u128);
                    assert_eq!(gval, expected, "limbs={:?}, m={}", limbs, m);
                }
            }
        }
    }

    // Test Mul for MarInt
    fn is_valid(mi: &MarInt) -> bool {
        // canonical zero
        if mi.sign == MZero {
            return mi.limbs.len() == 1 && mi.limbs[0] == 0;
        }
        // non-zero must not end with 0 limb
        mi.limbs.len() >= 1 && *mi.limbs.last().unwrap() != 0
    }


    fn mi_pos(limbs: &[u64]) -> MarInt {
        MarInt { sign: MPos, limbs: limbs.to_vec() }
    }
    fn mi_neg(limbs: &[u64]) -> MarInt {
        MarInt { sign: MNeg, limbs: limbs.to_vec() }
    }
    fn mi_zero() -> MarInt {
        MarInt { sign: MZero, limbs: MarInt::zero_limbs() }
    }

    // Convert a small MarInt (<=2 limbs) to i128 for cross-check tests.
    fn to_i128(mi: &MarInt) -> Option<i128> {
        if mi.limbs.len() > 2 {
            return None;
        }
        let mut mag: u128 = 0;
        if mi.limbs.len() >= 1 { mag += mi.limbs[0] as u128; }
        if mi.limbs.len() == 2 { mag += (mi.limbs[1] as u128) << 64; }

        match mi.sign {
            MZero => Some(0),
            MPos => {
                if mag <= i128::MAX as u128 { Some(mag as i128) } else { None }
            }
            MNeg => {
                // allow -i128::MAX..-1; disallow magnitude > i128::MAX+1 etc.
                if mag <= (i128::MAX as u128) + 1 { Some(-(mag as i128)) } else { None }
            }
        }
    }

    #[test]
    fn test_add_zero_rules() {
        let z = mi_zero();
        let a = mi_pos(&[5]);

        assert_eq!((a.clone() + z.clone()).sign, MPos);
        assert_eq!((a.clone() + z.clone()).limbs, vec![5]);

        assert_eq!((z.clone() + a.clone()).sign, MPos);
        assert_eq!((z + a).limbs, vec![5]);
    }

    #[test]
    fn test_add_same_sign_adds_magnitudes() {
        // 7 + 9 = 16
        let a = mi_pos(&[7]);
        let b = mi_pos(&[9]);
        let c = a + b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![16]);

        // -7 + -9 = -16
        let a = mi_neg(&[7]);
        let b = mi_neg(&[9]);
        let c = a + b;
        assert_eq!(c.sign, MNeg);
        assert_eq!(c.limbs, vec![16]);
    }

    #[test]
    fn test_add_opposite_sign_subtracts_magnitudes() {
        // 9 + (-7) = 2
        let a = mi_pos(&[9]);
        let b = mi_neg(&[7]);
        let c = a + b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![2]);

        // 7 + (-9) = -2
        let a = mi_pos(&[7]);
        let b = mi_neg(&[9]);
        let c = a + b;
        assert_eq!(c.sign, MNeg);
        assert_eq!(c.limbs, vec![2]);
    }

    #[test]
    fn test_add_equal_magnitude_opposite_sign_is_zero() {
        let a = mi_pos(&[123456]);
        //let b = mi_neg(&[123456]);
        let b= -a.clone();
        let c = a + b;

        assert_eq!(c.sign, MZero);
        assert_eq!(c.limbs, MarInt::zero_limbs());
    }
    #[test]
    fn test_add_carry_propagation() {
        // (2^64 - 1) + 1 = 2^64 => limbs [0,1]
        let a = mi_pos(&[u64::MAX]);
        let b = mi_pos(&[1]);
        let c = a + b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![0, 1]);
        assert!(is_valid(&c));

        // (2^128 - 1) + 1 = 2^128 => limbs [0,0,1]
        let a = mi_pos(&[u64::MAX, u64::MAX]);
        let b = mi_pos(&[1]);
        let c = a + b;
        println!("c.limbs = {:?}", c.limbs);
        assert_eq!(c.sign, MPos);
        assert!(is_valid(&c));
    }

    #[test]
    fn test_mul_zero_rules() {
        let z = mi_zero();
        let a = mi_pos(&[5]);
        let b = mi_neg(&[7, 2]);

        let r1 = a.clone() * z.clone();
        assert_eq!(r1.sign, MZero);
        assert_eq!(r1.limbs, MarInt::zero_limbs());

        let r2 = z.clone() * b.clone();
        assert_eq!(r2.sign, MZero);
        assert_eq!(r2.limbs, MarInt::zero_limbs());

        let r3 = z.clone() * z;
        assert_eq!(r3.sign, MZero);
        assert_eq!(r3.limbs, MarInt::zero_limbs());
    }

    #[test]
    fn test_mul_sign_rules() {
        // 3 * 5 = 15
        let a = mi_pos(&[3]);
        let b = mi_pos(&[5]);
        let c = a * b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![15]);

        // 3 * (-5) = -15
        let a = mi_pos(&[3]);
        let b = mi_neg(&[5]);
        let c = a * b;
        assert_eq!(c.sign, MNeg);
        assert_eq!(c.limbs, vec![15]);

        // (-3) * (-5) = 15
        let a = mi_neg(&[3]);
        let b = mi_neg(&[5]);
        let c = a * b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![15]);

        // Ensure no negative zero
        let z = mi_zero();
        let a = mi_neg(&[123]);
        let c = a * z;
        assert_eq!(c.sign, MZero);
        assert_eq!(c.limbs, MarInt::zero_limbs());
    }

    #[test]
    fn test_mul_single_limb_edge_cases() {
        // (2^64 - 1) * 2 = 2^65 - 2 => [u64::MAX - 1, 1]
        let a = mi_pos(&[u64::MAX]);
        let b = mi_pos(&[2]);
        let c = a * b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![u64::MAX - 1, 1]);
        assert!(is_valid(&c));

        // (2^64 - 1) * (2^64 - 1) = 2^128 - 2^65 + 1 => [1, u64::MAX - 1]
        let a = mi_pos(&[u64::MAX]);
        let b = mi_pos(&[u64::MAX]);
        let c = a * b;
        assert_eq!(c.limbs, vec![1, u64::MAX - 1]);
        assert!(is_valid(&c));
    }

    #[test]
    fn test_mul_multi_limb_known_values() {
        // (2^64) * (2^64) = 2^128 => limbs [0,0,1]
        let a = mi_pos(&[0, 1]); // 2^64
        let b = mi_pos(&[0, 1]); // 2^64
        let c = a * b;
        assert_eq!(c.sign, MPos);
        println!("\x1b[34mDEBUG\x1b[0m [test_mul_multi_limb_known_values] c.limbs = {:?}", c.limbs);
        assert_eq!(c.limbs, vec![0, 0, 1]);
        assert!(is_valid(&c));

        // (2^128 - 1) * 2 = 2^129 - 2 => [u64::MAX - 1, u64::MAX, 1]
        let a = mi_pos(&[u64::MAX, u64::MAX]); // 2^128 - 1
        let b = mi_pos(&[2]);
        let c = a * b;
        assert_eq!(c.limbs, vec![u64::MAX - 1, u64::MAX, 1]);
        assert!(is_valid(&c));
    }

    #[test]
    fn test_mul_commutative_small() {
        let vals = [
            mi_zero(),
            mi_pos(&[1]),
            mi_pos(&[7]),
            mi_neg(&[7]),
            mi_pos(&[u64::MAX]),
            mi_pos(&[1, 1]),
            mi_neg(&[2, 3]),
        ];

        for a in &vals {
            for b in &vals {
                let ab = a.clone() * b.clone();
                let ba = b.clone() * a.clone();
                assert_eq!(ab.sign, ba.sign, "a={:?}, b={:?}", a, b);
                assert_eq!(ab.limbs, ba.limbs, "a={:?}, b={:?}", a, b);
                assert!(is_valid(&ab));
            }
        }
    }

    #[test]
    fn test_mul_matches_i128_for_small_values() {
        // Cross-check against i128 for small numbers parsed from decimal strings.
        // This uses your FromStr implementation if present; otherwise, build from limbs manually.
        let vals: [i128; 15] = [
            -1000, -123, -7, -2, -1, 0, 1, 2, 7, 123, 999,
            (1i128 << 31) - 1,
            -((1i128 << 31) - 1),
            (1i128 << 63) - 1,
            -((1i128 << 63) - 1),
        ];

        for &x in &vals {
            for &y in &vals {
                // Prefer parsing to exercise your parser too; replace if you haven't implemented it yet.
                let a: MarInt = x.to_string().parse().unwrap();
                let b: MarInt = y.to_string().parse().unwrap();
                let c = a.clone() * b.clone();

                let ax = to_i128(&a).unwrap();
                let by = to_i128(&b).unwrap();
                let expected = ax * by;

                let got = to_i128(&c).expect("result too large for i128 in this test set");
                assert_eq!(got, expected, "a={:?}, b={:?}, c={:?}", a, b, c);
                assert!(is_valid(&c));
            }
        }
    }

    #[test]
    fn test_mul_associative_small() {
        // (a*b)*c == a*(b*c) for a small set
        let vals = [
            mi_zero(),
            mi_pos(&[1]),
            mi_pos(&[2]),
            mi_pos(&[7]),
            mi_neg(&[3]),
            mi_pos(&[u64::MAX]),
            mi_pos(&[5, 1]),
        ];

        for a in &vals {
            for b in &vals {
                for c in &vals {
                    let left = (a.clone() * b.clone()) * c.clone();
                    let right = a.clone() * (b.clone() * c.clone());
                    assert_eq!(left.sign, right.sign, "a={:?}, b={:?}, c={:?}", a, b, c);
                    assert_eq!(left.limbs, right.limbs, "a={:?}, b={:?}, c={:?}", a, b, c);
                    assert!(is_valid(&left));
                }
            }
        }
    }

    // For Sub Operation

    #[test]
    fn test_sub_zero_rules() {
        let z = mi_zero();
        let a = mi_pos(&[5]);
        let b = mi_neg(&[7, 2]);

        // a - 0 = a
        let r1 = a.clone() - z.clone();
        assert_eq!(r1.sign, a.sign);
        assert_eq!(r1.limbs, a.limbs);

        // 0 - a = -a
        let r2 = z.clone() - a.clone();
        assert_eq!(r2.sign, MNeg);
        assert_eq!(r2.limbs, a.limbs);

        // 0 - (-b) = +b
        let r3 = z - b.clone();
        assert_eq!(r3.sign, MPos);
        assert_eq!(r3.limbs, b.limbs);

        assert!(is_valid(&r1));
        assert!(is_valid(&r2));
        assert!(is_valid(&r3));
    }

    #[test]
    fn test_sub_same_sign_basic() {
        // 9 - 7 = 2
        let a = mi_pos(&[9]);
        let b = mi_pos(&[7]);
        let c = a - b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![2]);
        assert!(is_valid(&c));

        // 7 - 9 = -2
        let a = mi_pos(&[7]);
        let b = mi_pos(&[9]);
        let c = a - b;
        assert_eq!(c.sign, MNeg);
        assert_eq!(c.limbs, vec![2]);
        assert!(is_valid(&c));

        // (-9) - (-7) = -(9-7) = -2
        let a = mi_neg(&[9]);
        let b = mi_neg(&[7]);
        let c = a - b;
        assert_eq!(c.sign, MNeg);
        assert_eq!(c.limbs, vec![2]);
        assert!(is_valid(&c));

        // (-7) - (-9) = +2
        let a = mi_neg(&[7]);
        let b = mi_neg(&[9]);
        let c = a - b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![2]);
        assert!(is_valid(&c));
    }

    #[test]
    fn test_sub_opposite_sign_adds_magnitudes() {
        // 7 - (-9) = 16
        let a = mi_pos(&[7]);
        let b = mi_neg(&[9]);
        let c = a - b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![16]);

        // (-7) - (9) = -16
        let a = mi_neg(&[7]);
        let b = mi_pos(&[9]);
        let c = a - b;
        assert_eq!(c.sign, MNeg);
        assert_eq!(c.limbs, vec![16]);
    }

    #[test]
    fn test_sub_equal_is_zero() {
        let a = mi_pos(&[123456]);
        let b = mi_pos(&[123456]);
        let c = a - b;
        assert_eq!(c.sign, MZero);
        assert_eq!(c.limbs, MarInt::zero_limbs());
        assert!(is_valid(&c));

        // (-x) - (-x) = 0
        let a = mi_neg(&[42, 7]);
        let b = mi_neg(&[42, 7]);
        let c = a - b;
        assert_eq!(c.sign, MZero);
        assert_eq!(c.limbs, MarInt::zero_limbs());
        assert!(is_valid(&c));
    }

    #[test]
    fn test_sub_borrow_propagation() {
        // 2^64 - 1 = [u64::MAX]
        // 2^64 = [0,1]
        // 2^64 - 1 = 2^64 + 0 - 1 => [u64::MAX]
        let a = mi_pos(&[0, 1]); // 2^64
        let b = mi_pos(&[1]);
        let c = a - b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![u64::MAX]);
        assert!(is_valid(&c));

        // (2^128) - 1 = [u64::MAX, u64::MAX]
        let a = mi_pos(&[0, 0, 1]); // 2^128
        let b = mi_pos(&[1]);
        let c = a - b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![u64::MAX, u64::MAX]);
        assert!(is_valid(&c));

        // Borrow across middle zeros: [0,0,5] - [1] => [u64::MAX, u64::MAX, 4]
        let a = mi_pos(&[0, 0, 5]);
        let b = mi_pos(&[1]);
        let c = a - b;
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![u64::MAX, u64::MAX, 4]);
        assert!(is_valid(&c));
    }

    #[test]
    fn test_sub_matches_i128_for_small_values() {
        let vals: [i128; 15] = [
            -1000, -123, -7, -2, -1, 0, 1, 2, 7, 123, 999,
            (1i128 << 31) - 1,
            -((1i128 << 31) - 1),
            (1i128 << 63) - 1,
            -((1i128 << 63) - 1),
        ];

        for &x in &vals {
            for &y in &vals {
                let a: MarInt = x.to_string().parse().unwrap();
                let b: MarInt = y.to_string().parse().unwrap();
                let c = a.clone() - b.clone();

                let ax = to_i128(&a).unwrap();
                let by = to_i128(&b).unwrap();
                let expected = ax - by;

                let got = to_i128(&c).expect("result too large for i128 in this test set");
                assert_eq!(got, expected, "a={:?}, b={:?}, c={:?}", a, b, c);
                assert!(is_valid(&c));
            }
        }
    }

    #[test]
    fn test_sub_relation_with_add_and_neg() {
        // a - b == a + (-b)
        let vals = [
            mi_zero(),
            mi_pos(&[1]),
            mi_pos(&[7]),
            mi_neg(&[7]),
            mi_pos(&[u64::MAX]),
            mi_neg(&[2, 3]),
        ];

        for a in &vals {
            for b in &vals {
                let left = a.clone() - b.clone();
                let right = a.clone() + (MarInt { sign: -b.sign, limbs: b.limbs.clone() }); // if you implement Neg, prefer -b.clone()
                assert_eq!(left.sign, right.sign, "a={:?}, b={:?}", a, b);
                assert_eq!(left.limbs, right.limbs, "a={:?}, b={:?}", a, b);
                assert!(is_valid(&left));
            }
        }
    }
}
