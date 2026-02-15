use marnd::entropy::mosentropy::MOSEntropy;
use std::io::ErrorKind;

#[cfg(any(target_os = "macos", target_os = "linux"))]
use marnd::entropy::mosentropy::MOSRng;

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn mosrng_fill_bytes_populates_buffer() {
    let mut rng = MOSRng;
    let mut out = [0u8; 64];
    rng.fill_bytes(&mut out).expect("fill_bytes should succeed");
    assert!(out.iter().any(|&b| b != 0), "unexpected all-zero output");
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn mosrng_fill_bytes_accepts_empty_slice() {
    let mut rng = MOSRng;
    let mut out = [0u8; 0];
    rng.fill_bytes(&mut out)
        .expect("fill_bytes should accept empty output");
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn mosrng_next_u64_and_seed256_succeed() {
    let mut rng = MOSRng;

    let n = rng.next_u64().expect("next_u64 should succeed");
    let seed = rng.seed256().expect("seed256 should succeed");

    assert!(n <= u64::MAX);
    assert_eq!(seed.len(), 32);
    assert!(
        seed.iter().any(|&b| b != 0),
        "unexpected all-zero 256-bit seed"
    );
}

#[test]
fn default_mosentropy_impl_returns_unsupported() {
    struct DummyEntropy;
    impl MOSEntropy for DummyEntropy {}

    let mut entropy = DummyEntropy;
    let mut out = [0u8; 8];
    let err = entropy
        .fill_bytes(&mut out)
        .expect_err("default impl should be unsupported");

    assert_eq!(err.kind(), ErrorKind::Unsupported);
    assert!(err.to_string().contains("DummyEntropy"));
}
