// tests/unittest/operators_add_sub.rs
//
// Integration tests for MarInt operator suite:
// - Add / Sub
// - AddAssign / SubAssign
// - owned + borrowed combinations
// - canonical zero invariants
// - randomized cross-check with i128
//
// Crate name: marint

#[cfg(test)]
mod operators_ref_add_sub_tests {
    use super::*;

    use marint::marint::MarInt;
    use marint::sign::MSgn;
    use marint::sign::MSgn::{MNeg, MPos, MZero};


    fn mk(sign: MSgn, limbs: Vec<u64>) -> MarInt {
        let mut x = MarInt { sign, limbs };
        x.normalize();
        x
    }

    fn assert_is_canonical_zero(x: &MarInt) {
        assert_eq!(x.sign, MZero, "zero must have sign MZero");
        assert!(x.is_zero(), "zero must satisfy is_zero()");
        assert_eq!(x.limbs.len(), 1, "zero limbs must be [0]");
        assert_eq!(x.limbs[0], 0, "zero limbs must be [0]");
    }

    /* -----------------------------
     * Addition tests
     * ----------------------------- */

    #[test]
    fn add_ref_ref_does_not_move_operands() {
        let a = mk(MPos, vec![5]);
        let b = mk(MNeg, vec![2]);

        let c = &a + &b;

        assert_eq!(a.limbs, vec![5]);
        assert_eq!(b.limbs, vec![2]);

        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![3]);
    }

    #[test]
    fn add_move_left_borrow_right() {
        let a = mk(MPos, vec![10]);
        let b = mk(MPos, vec![20]);

        let c = a + &b;

        assert_eq!(b.limbs, vec![20]);
        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![30]);
    }

    #[test]
    fn add_move_both() {
        let a = mk(MPos, vec![7]);
        let b = mk(MNeg, vec![3]);

        let c = a + b;

        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![4]);
    }

    #[test]
    fn add_assign_borrow_rhs() {
        let mut a = mk(MPos, vec![1]);
        let b = mk(MPos, vec![2]);

        a += &b;

        assert_eq!(b.limbs, vec![2]);
        assert_eq!(a.limbs, vec![3]);
    }

    #[test]
    fn add_assign_zero_fast_paths() {
        let mut a = mk(MZero, vec![0]);
        let b = mk(MPos, vec![42]);

        a += &b;
        assert_eq!(a.sign, MPos);
        assert_eq!(a.limbs, vec![42]);

        let mut x = mk(MPos, vec![99]);
        let z = mk(MZero, vec![0]);

        x += &z;
        assert_eq!(x.limbs, vec![99]);
    }

    #[test]
    fn add_opposites_produce_canonical_zero() {
        let a = mk(MPos, vec![123456789]);
        let b = mk(MNeg, vec![123456789]);

        let c1 = &a + &b;
        assert_is_canonical_zero(&c1);

        let mut c2 = a.clone();
        c2 += &b;
        assert_is_canonical_zero(&c2);
    }

    #[test]
    fn add_with_carry_across_limbs() {
        let a = mk(MPos, vec![u64::MAX]);
        let b = mk(MPos, vec![1]);

        let c = &a + &b;

        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![0, 1]);
    }

    /* -----------------------------
     * Subtraction tests
     * ----------------------------- */

    #[test]
    fn sub_ref_ref_does_not_move_operands() {
        let a = mk(MPos, vec![9]);
        let b = mk(MPos, vec![4]);

        let c = &a - &b;

        assert_eq!(a.limbs, vec![9]);
        assert_eq!(b.limbs, vec![4]);

        assert_eq!(c.limbs, vec![5]);
    }

    #[test]
    fn sub_move_left_borrow_right() {
        let a = mk(MNeg, vec![10]);
        let b = mk(MPos, vec![2]);

        let c = a - &b;

        assert_eq!(c.sign, MNeg);
        assert_eq!(c.limbs, vec![12]);
    }

    #[test]
    fn sub_assign_borrow_rhs() {
        let mut a = mk(MPos, vec![100]);
        let b = mk(MPos, vec![1]);

        a -= &b;

        assert_eq!(a.limbs, vec![99]);
        assert_eq!(b.limbs, vec![1]);
    }

    #[test]
    fn sub_assign_zero_fast_paths() {
        let mut a = mk(MZero, vec![0]);
        let b = mk(MPos, vec![5]);

        a -= &b;
        assert_eq!(a.sign, MNeg);
        assert_eq!(a.limbs, vec![5]);

        let mut x = mk(MPos, vec![77]);
        let z = mk(MZero, vec![0]);

        x -= &z;
        assert_eq!(x.limbs, vec![77]);
    }

    #[test]
    fn subtraction_to_zero_is_canonical() {
        let a = mk(MPos, vec![0xDEADBEEF]);
        let b = mk(MPos, vec![0xDEADBEEF]);

        let c = &a - &b;
        assert_is_canonical_zero(&c);
    }

    #[test]
    fn sub_with_borrow_across_limbs() {
        let a = mk(MPos, vec![0, 1]); // 2^64
        let b = mk(MPos, vec![1]);

        let c = &a - &b;

        assert_eq!(c.sign, MPos);
        assert_eq!(c.limbs, vec![u64::MAX]);
    }

    /* -----------------------------
     * Randomized cross-check (i128)
     * ----------------------------- */

    #[test]
    fn randomized_small_vectors_compare_with_i128_when_fit() {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;

        let mut rng = StdRng::seed_from_u64(12345);

        for _ in 0..5_000 {
            let xa: i128 = rng.gen_range(-1_000_000..=1_000_000);
            let xb: i128 = rng.gen_range(-1_000_000..=1_000_000);

            let a = MarInt::from_i128(xa);
            let b = MarInt::from_i128(xb);

            // Addition
            let expected_add = MarInt::from_i128(xa + xb);

            let c1 = &a + &b;
            let c2 = a.clone() + &b;
            let mut c3 = a.clone();
            c3 += &b;

            assert_eq!(c1.sign, expected_add.sign);
            assert_eq!(c1.limbs, expected_add.limbs);

            assert_eq!(c2.sign, expected_add.sign);
            assert_eq!(c2.limbs, expected_add.limbs);

            assert_eq!(c3.sign, expected_add.sign);
            assert_eq!(c3.limbs, expected_add.limbs);

            // Subtraction
            let expected_sub = MarInt::from_i128(xa - xb);

            let d1 = &a - &b;
            let d2 = a.clone() - &b;
            let mut d3 = a.clone();
            d3 -= &b;

            assert_eq!(d1.sign, expected_sub.sign);
            assert_eq!(d1.limbs, expected_sub.limbs);

            assert_eq!(d2.sign, expected_sub.sign);
            assert_eq!(d2.limbs, expected_sub.limbs);

            assert_eq!(d3.sign, expected_sub.sign);
            assert_eq!(d3.limbs, expected_sub.limbs);

            if xa + xb == 0 {
                assert_is_canonical_zero(&c1);
            }
            if xa - xb == 0 {
                assert_is_canonical_zero(&d1);
            }
        }
    }
}
