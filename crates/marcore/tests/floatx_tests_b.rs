use marcore::floatx::FloatX;
use marcore::{f32_isclose, f64_isclose};

/// ===== f64 basic behavior =====

#[test]
fn f64_basic_default() {
    // classic floating example
    let a = 0.1 + 0.2;
    let b = 0.3;

    assert!(f64_isclose!(a, b));
}

#[test]
fn f64_exact_equality() {
    assert!(f64_isclose!(1.0, 1.0));
    assert!(f64_isclose!(0.0, -0.0)); // signed zero
}

#[test]
fn f64_nan_rules() {
    assert!(!f64_isclose!(f64::NAN, f64::NAN));
    assert!(!f64_isclose!(f64::NAN, 1.0));
    assert!(!f64_isclose!(1.0, f64::NAN));
}

#[test]
fn f64_infinity_rules() {
    assert!(f64_isclose!(f64::INFINITY, f64::INFINITY));
    assert!(f64_isclose!(f64::NEG_INFINITY, f64::NEG_INFINITY));

    assert!(!f64_isclose!(f64::INFINITY, f64::NEG_INFINITY));
    assert!(!f64_isclose!(f64::INFINITY, 1.0));
}

#[test]
fn f64_relative_region() {
    // large magnitude: relative tolerance dominates
    let a = 1e9;
    let b = a + 0.5; // within 1e-9 * 1e9 = 1.0
    assert!(f64_isclose!(a, b));

    let c = a + 2.0; // outside tolerance
    assert!(!f64_isclose!(a, c));
}

#[test]
fn f64_absolute_region() {
    // near zero: absolute tolerance dominates
    assert!(f64_isclose!(1e-13, 0.0));
    assert!(f64_isclose!(-1e-13, 0.0));

    assert!(!f64_isclose!(1e-10, 0.0));
}

#[test]
fn f64_custom_tolerances() {
    assert!(f64_isclose!(1.0, 1.0001, 1e-3));
    assert!(f64_isclose!(0.0, 1e-6, 1e-3, 1e-5));
}

/// ===== f32 behavior =====

#[test]
fn f32_basic_default() {
    let a: f32 = 0.1 + 0.2;
    let b: f32 = 0.3;

    assert!(f32_isclose!(a, b));
}

#[test]
fn f32_nan_inf() {
    assert!(!f32_isclose!(f32::NAN, f32::NAN));
    assert!(f32_isclose!(f32::INFINITY, f32::INFINITY));
    assert!(f32_isclose!(f32::NEG_INFINITY, f32::NEG_INFINITY));
    assert!(!f32_isclose!(f32::INFINITY, f32::NEG_INFINITY));
}

#[test]
fn f32_relative_region() {
    let a: f32 = 1e6;
    let b: f32 = a + 0.05;
    assert!(f32_isclose!(a, b));

    let c: f32 = a + 2.0;
    assert!(!f32_isclose!(a, c));
}

#[test]
fn f32_absolute_region() {
    assert!(f32_isclose!(1e-9f32, 0.0f32));
    assert!(!f32_isclose!(1e-6f32, 0.0f32));
}

/// ===== Trait API consistency =====

#[test]
fn trait_vs_macro_consistency_f64() {
    let a = 1.23456789;
    let b = 1.23456780;

    let via_trait = a.isclose(
        &b,
        <f64 as FloatX>::DEFAULT_REL_TOL,
        <f64 as FloatX>::DEFAULT_ABS_TOL,
    );

    let via_macro = f64_isclose!(a, b);

    assert_eq!(via_trait, via_macro);
}

#[test]
fn symmetry_property() {
    let a = 0.123456789;
    let b = 0.123456780;

    assert_eq!(f64_isclose!(a, b), f64_isclose!(b, a));
}
