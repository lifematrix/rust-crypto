// tests/div_rem_ops.rs

use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

use num_bigint::BigInt;
use num_traits::{Zero, Signed, ToPrimitive};

// Change this to your actual crate/module path.
use marint::marint::MarInt;
use marint::sign::MSgn;
use marint::sign::MSgn::{MNeg, MZero, MPos};

fn assert_normalized_le(limbs: &[u64]) {
    assert!(!limbs.is_empty(), "limbs must not be empty");
    if limbs.len() > 1 {
        assert_ne!(*limbs.last().unwrap(), 0, "MS limb must not be 0");
    }
}

/// Convert MarInt (sign + LE u64 limbs) -> BigInt (exact).
fn marint_to_bigint(x: &MarInt) -> BigInt {
    // Build a big-endian magnitude (bytes) from LE limbs.
    // Each limb contributes 8 bytes in little-endian; BigInt::from_bytes_le expects LE bytes.
    let mut bytes = Vec::<u8>::with_capacity(x.limbs.len() * 8);
    for &w in &x.limbs {
        bytes.extend_from_slice(&w.to_le_bytes());
    }

    let mag = BigInt::from_bytes_le(num_bigint::Sign::Plus, &bytes);

    match x.sign {
        MZero => BigInt::zero(),
        MPos => mag,
        MNeg => -mag,
    }
}

/// Convert BigInt -> MarInt (sign + LE u64 limbs), normalized.
fn bigint_to_marint(x: &BigInt) -> MarInt {
    if x.is_zero() {
        return MarInt { sign: MZero, limbs: vec![0] };
    }
    let sign = if x.is_negative() { MNeg } else { MPos };
    let (_s, bytes) = x.abs().to_bytes_le();

    // Pack bytes into u64 limbs (little-endian).
    let mut limbs = Vec::<u64>::with_capacity((bytes.len() + 7) / 8);
    for chunk in bytes.chunks(8) {
        let mut buf = [0u8; 8];
        buf[..chunk.len()].copy_from_slice(chunk);
        limbs.push(u64::from_le_bytes(buf));
    }

    // Normalize to your canonical representation.
    MarInt::normalize_limbs(&mut limbs);

    let mut out = MarInt { sign, limbs };
    out.normalize();
    out
}

fn rand_nonzero_marint(rng: &mut StdRng, n_limbs: usize) -> MarInt {
    let mut limbs = vec![0u64; n_limbs.max(1)];
    for w in &mut limbs {
        *w = rng.next_u64();
    }

    // Make sure it's not zero by setting the top limb non-zero.
    if let Some(last) = limbs.last_mut() {
        if *last == 0 {
            *last = 1;
        }
    }
    MarInt::normalize_limbs(&mut limbs);

    let sign_pick = (rng.next_u32() % 3) as i32;
    let sign = match sign_pick {
        0 => MPos,
        1 => MNeg,
        _ => MPos,
    };

    let mut x = MarInt { sign, limbs };
    x.normalize();
    x
}

fn rand_marint(rng: &mut StdRng, n_limbs: usize) -> MarInt {
    let mut limbs = vec![0u64; n_limbs.max(1)];
    for w in &mut limbs {
        *w = rng.next_u64();
    }
    MarInt::normalize_limbs(&mut limbs);

    let sign_pick = (rng.next_u32() % 3) as i32;
    let sign = match sign_pick {
        0 => MZero, // allow zeros sometimes
        1 => MPos,
        _ => MNeg,
    };

    let mut x = MarInt { sign, limbs };
    x.normalize();
    x
}

fn check_div_rem_case(a: &MarInt, b: &MarInt) {
    if b.is_zero() {
        return;
    }

    // Use your operators
    let q = a / b;
    let r = a % b;

    // Basic invariants: normalized outputs
    assert_normalized_le(&q.limbs);
    assert_normalized_le(&r.limbs);
    println!("[check_div_rem_case] a = {:?}", a);
    println!("[check_div_rem_case] b = {:?}", b);
    println!("[check_div_rem_case] q = {:?}", q);
    println!("[check_div_rem_case] r = {:?}", r);

    // Compare to BigInt oracle
    let a_bi = marint_to_bigint(a);
    let b_bi = marint_to_bigint(b);

    let q_bi = &a_bi / &b_bi; // trunc toward zero
    let r_bi = &a_bi % &b_bi; // remainder sign follows dividend (Rust/BigInt style)

    println!("[check_div_rem_case] a_bi = {:?}", a_bi);
    println!("[check_div_rem_case] b_bi = {:?}", b_bi);
    println!("[check_div_rem_case] q_bi = {:?}", q_bi);
    println!("[check_div_rem_case] r_bi = {:?}", r_bi);

    let q_expected = bigint_to_marint(&q_bi);
    let r_expected = bigint_to_marint(&r_bi);

    assert_eq!(q.sign, q_expected.sign, "quot sign mismatch: a={a_bi} b={b_bi}");
    assert_eq!(q.limbs, q_expected.limbs, "quot limbs mismatch: a={a_bi} b={b_bi}");

    assert_eq!(r.sign, r_expected.sign, "rem sign mismatch: a={a_bi} b={b_bi}");
    assert_eq!(r.limbs, r_expected.limbs, "rem limbs mismatch: a={a_bi} b={b_bi}");

    // Reconstruction property (also as a sanity check): a = q*b + r
    let recon = marint_to_bigint(&q) * &b_bi + marint_to_bigint(&r);
    assert_eq!(recon, a_bi, "reconstruction failed: a != q*b + r");

    // Remainder magnitude constraint: |r| < |b|
    let rb = marint_to_bigint(&r).abs();
    let bb = b_bi.abs();
    assert!(rb < bb, "remainder magnitude not < divisor: |r|={rb} |b|={bb}");
}

#[test]
fn div_rem_small_known_values() {
    let cases: &[(i128, i128)] = &[
        (0, 1),
        (1, 1),
        (7, 3),
        (10, 3),
        (-10, 3),
        (10, -3),
        (-10, -3),
        (i128::MAX, 7),
        (i128::MIN + 1, 5),
    ];

    for &(a, b) in cases {
        if b == 0 { continue; }
        let aa = MarInt::from_i128(a);
        let bb = MarInt::from_i128(b);
        check_div_rem_case(&aa, &bb);
    }
}

#[test]
fn div_rem_many_limb_random_stress() {
    // Deterministic seed => reproducible failures
    let mut rng = StdRng::seed_from_u64(0xC0FFEE_1234_5678);

    // Try a range of sizes, including "many limbs"
    let limb_sizes = [1usize, 2, 3, 4, 8, 16, 32, 64];

    for &na in &limb_sizes {
        for &nb in &limb_sizes {
            for _ in 0..200 {
                let a = rand_marint(&mut rng, na);
                let b = rand_nonzero_marint(&mut rng, nb);
                check_div_rem_case(&a, &b);
            }
        }
    }
}

#[test]
fn div_rem_edge_cases_with_leading_zero_inputs() {
    // Even if your MarInt invariants keep values normalized,
    // this test helps catch bugs if someone constructs unnormalized values.

    let mut a = MarInt::from_i128(123456789);
    a.limbs.push(0);
    a.limbs.push(0);
    a.normalize();

    let mut b = MarInt::from_i128(12345);
    b.limbs.push(0);
    b.normalize();

    check_div_rem_case(&a, &b);
}

#[test]
fn div_rem_divisor_one_and_minus_one_many_limbs() {
    let mut rng = StdRng::seed_from_u64(0xBADC0DE_9999);

    for _ in 0..200 {
        let a = rand_marint(&mut rng, 32);

        let one = MarInt::from_i128(1);
        let neg_one = MarInt::from_i128(-1);

        check_div_rem_case(&a, &one);
        check_div_rem_case(&a, &neg_one);
    }
}

#[test]
#[should_panic(expected = "Division by zero")]
fn div_by_zero_panics() {
    let a = MarInt::from_i128(123);
    let z = MarInt::zero();
    let _ = &a / &z;
}

#[test]
#[should_panic(expected = "Division by zero")]
fn rem_by_zero_panics() {
    let a = MarInt::from_i128(123);
    let z = MarInt::zero();
    let _ = &a % &z;
}
