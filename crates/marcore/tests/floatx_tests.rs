use marcore::floatx::*;
use marcore::{f64_eq, f32_eq};

#[test]
fn f64_basic_equality() {
    assert!(f64_eq!(1.0, 1.0));
    assert!(f64_eq!(0.0, -0.0));
    assert!(f64_eq!(0.1 + 0.2, 0.3));
}

#[test]
fn f64_nan_rules() {
    assert!(!f64_eq!(f64::NAN, f64::NAN));
    assert!(!f64_eq!(f64::NAN, 1.0));
}

#[test]
fn f64_infinities() {
    assert!(f64_eq!(f64::INFINITY, f64::INFINITY));
    assert!(!f64_eq!(f64::INFINITY, f64::NEG_INFINITY));
}

#[test]
fn f64_relative_behavior() {
    let a = 1e9;
    let b = a + 0.5;
    assert!(f64_eq!(a, b));

    let c = a + 2.0;
    assert!(!f64_eq!(a, c));
}

#[test]
fn f64_absolute_behavior() {
    assert!(f64_eq!(1e-13, 0.0));
    assert!(!f64_eq!(1e-11, 0.0));
}

#[test]
fn f64_custom_tolerances() {
    assert!(f64_eq!(1.0, 1.0001, 1e-3));
    assert!(f64_eq!(0.0, 1e-6, 1e-3, 1e-5));
}

// ===== f32 =====

#[test]
fn f32_basic() {
    assert!(f32_eq!(1.0f32, 1.0f32));
    assert!(f32_eq!(0.0f32, -0.0f32));

    let a: f32 = 0.1 + 0.2;
    let b: f32 = 0.3;
    assert!(f32_eq!(a, b));
}

#[test]
fn f32_nan_inf() {
    assert!(!f32_eq!(f32::NAN, f32::NAN));
    assert!(f32_eq!(f32::INFINITY, f32::INFINITY));
}

#[test]
fn f32_relative() {
    let a: f32 = 1e6;
    let b: f32 = a + 0.05;
    assert!(f32_eq!(a, b));

    let c: f32 = a + 2.0;
    assert!(!f32_eq!(a, c));
}

#[test]
fn macro_matches_impl() {
    let a = 1.23456789;
    let b = 1.23456780;

    assert_eq!(
        f64_eq!(a, b),
        eq_impl_f64(a, b,
            DEFAULT_REL_TOL_F64,
            DEFAULT_ABS_TOL_F64)
    );
}