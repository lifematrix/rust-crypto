use marcore::fmt_slice_debug;
use marcore::formatx::fmt_slice_compact;

#[test]
fn empty_slice_default() {
    let xs: Vec<i32> = vec![];
    assert_eq!(fmt_slice_compact(&xs, None), "[]");
    assert_eq!(fmt_slice_debug!(&xs), "[]");
}

#[test]
fn empty_slice_all() {
    let xs: Vec<i32> = vec![];
    assert_eq!(fmt_slice_compact(&xs, Some(usize::MAX)), "[]");
    assert_eq!(fmt_slice_debug!(&xs, ALL), "[]");
}

#[test]
fn single_element_default() {
    let xs = vec![42];
    assert_eq!(fmt_slice_compact(&xs, None), "[42]");
    assert_eq!(fmt_slice_debug!(&xs), "[42]");
}

#[test]
fn single_element_n0() {
    // max = 0: should show nothing (your current semantics)
    let xs = vec![42];
    assert_eq!(fmt_slice_compact(&xs, Some(0)), "[...]");
    assert_eq!(fmt_slice_debug!(&xs, 0), "[...]");
}

#[test]
fn single_element_n1() {
    let xs = vec![42];
    assert_eq!(fmt_slice_compact(&xs, Some(1)), "[42]");
    assert_eq!(fmt_slice_debug!(&xs, 1), "[42]");
}

#[test]
fn two_elements_default() {
    // default=3 but slice len=2 => show all
    let xs = vec![1, 2];
    assert_eq!(fmt_slice_compact(&xs, None), "[1, 2]");
    assert_eq!(fmt_slice_debug!(&xs), "[1, 2]");
}

#[test]
fn two_elements_n1() {
    // n=1: shows last only per your current code
    let xs = vec![1, 2];
    assert_eq!(fmt_slice_compact(&xs, Some(1)), "[1, ...]");
    assert_eq!(fmt_slice_debug!(&xs, 1), "[1, ...]");
}

#[test]
fn two_elements_n0() {
    let xs = vec![1, 2];
    assert_eq!(fmt_slice_compact(&xs, Some(0)), "[...]");
    assert_eq!(fmt_slice_debug!(&xs, 0), "[...]");
}

#[test]
fn three_elements_default_no_ellipsis() {
    let xs = vec![1, 2, 3];
    // default=3 => show all, no ellipsis
    assert_eq!(fmt_slice_compact(&xs, None), "[1, 2, 3]");
    assert_eq!(fmt_slice_debug!(&xs), "[1, 2, 3]");
}

#[test]
fn four_elements_default_has_ellipsis() {
    let xs = vec![1, 2, 3, 4];
    // default=3 => show first 2, ..., last
    assert_eq!(fmt_slice_compact(&xs, None), "[1, 2, ..., 4]");
    assert_eq!(fmt_slice_debug!(&xs), "[1, 2, ..., 4]");
}

#[test]
fn four_elements_all() {
    let xs = vec![1, 2, 3, 4];
    assert_eq!(fmt_slice_compact(&xs, Some(usize::MAX)), "[1, 2, 3, 4]");
    assert_eq!(fmt_slice_debug!(&xs, ALL), "[1, 2, 3, 4]");
}

#[test]
fn four_elements_n4_show_all() {
    let xs = vec![1, 2, 3, 4];
    assert_eq!(fmt_slice_compact(&xs, Some(4)), "[1, 2, 3, 4]");
    assert_eq!(fmt_slice_debug!(&xs, 4), "[1, 2, 3, 4]");
}

#[test]
fn four_elements_n3_truncated() {
    let xs = vec![1, 2, 3, 4];
    assert_eq!(fmt_slice_compact(&xs, Some(3)), "[1, 2, ..., 4]");
    assert_eq!(fmt_slice_debug!(&xs, 3), "[1, 2, ..., 4]");
}

#[test]
fn four_elements_n2_truncated() {
    let xs = vec![1, 2, 3, 4];
    // n=2: show first 1, ..., last
    assert_eq!(fmt_slice_compact(&xs, Some(2)), "[1, ..., 4]");
    assert_eq!(fmt_slice_debug!(&xs, 2), "[1, ..., 4]");
}

#[test]
fn four_elements_n1_last_only() {
    let xs = vec![1, 2, 3, 4];
    // n=1: show last only
    assert_eq!(fmt_slice_compact(&xs, Some(1)), "[1, ...]");
    assert_eq!(fmt_slice_debug!(&xs, 1), "[1, ...]");
}

#[test]
fn n_greater_than_len_equivalent_to_all() {
    let xs = vec![10, 20, 30];
    assert_eq!(fmt_slice_compact(&xs, Some(999)), "[10, 20, 30]");
    assert_eq!(fmt_slice_debug!(&xs, 999), "[10, 20, 30]");
}

#[test]
fn works_for_non_numeric_display() {
    let xs = vec!["a", "b", "c", "d"];
    assert_eq!(fmt_slice_compact(&xs, None), "[a, b, ..., d]");
    assert_eq!(fmt_slice_compact(&xs, Some(2)), "[a, ..., d]");
    assert_eq!(fmt_slice_compact(&xs, Some(usize::MAX)), "[a, b, c, d]");
}
