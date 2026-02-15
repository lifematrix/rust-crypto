// pub const DEFAULT_REL_TOL_F64: f64 = 1e-9;
// pub const DEFAULT_ABS_TOL_F64: f64 = 1e-12;

// pub const DEFAULT_REL_TOL_F32: f32 = 1e-6;
// pub const DEFAULT_ABS_TOL_F32: f32 = 1e-8;

// /// Adopt the logic in Python implementation deccribed in [PEP 485](https://peps.python.org/pep-0485/)
// /// rel_tol: relative tolerance,
// /// abs_tol: absolute value tolerate
// #[inline]
// pub fn eq_impl_f64(a: f64, b: f64, rel_tol: f64, abs_tol: f64) -> bool {
//     if a == b {
//         return true;
//     }

//     if a.is_nan() || b.is_nan() {
//         return false;
//     }

//     if a.is_infinite() || b.is_infinite() {
//         return false;
//     }

//     let abs_diff = (a-b).abs();
//     let max_tol = abs_tol.max(a.abs().max(b.abs())*rel_tol);

//     abs_diff <= max_tol
// }

// #[inline]
// pub fn eq_impl_f32(a: f32, b: f32, rel_tol: f32, abs_tol: f32) -> bool {
//     if a == b {
//         return true;
//     }

//     if a.is_nan() || b.is_nan() {
//         return false;
//     }

//     if a.is_infinite() || b.is_infinite() {
//         return false;
//     }

//     let abs_diff = (a-b).abs();
//     let max_tol = abs_tol.max(a.abs().max(b.abs())*rel_tol);

//     abs_diff <= max_tol
// }

// #[macro_export]
// macro_rules! f64_eq {
//     // defaults
//     ($a:expr, $b:expr) => {
//         $crate::eq_impl_f64(
//             $a,
//             $b,
//             $crate::floatx::DEFAULT_REL_TOL_F64,
//             $crate::floatx::DEFAULT_ABS_TOL_F64,
//         )
//     };

//     // override rel only
//     ($a:expr, $b:expr, $rel:expr) => {
//         $crate::eq_impl_f64(
//             $a,
//             $b,
//             $rel,
//             $crate::floatx::DEFAULT_ABS_TOL_F64,
//         )
//     };

//     // override rel + abs
//     ($a:expr, $b:expr, $rel:expr, $abs:expr) => {
//         $crate::eq_impl_f64($a, $b, $rel, $abs)
//     };
// }

// #[macro_export]
// macro_rules! f32_eq {
//     // defaults
//     ($a:expr, $b:expr) => {
//         $crate::eq_impl_f32(
//             $a,
//             $b,
//             $crate::floatx::DEFAULT_REL_TOL_F32,
//             $crate::floatx::DEFAULT_ABS_TOL_F32,
//         )
//     };

//     // override rel only
//     ($a:expr, $b:expr, $rel:expr) => {
//         $crate::eq_impl_f32(
//             $a,
//             $b,
//             $rel,
//             $crate::floatx::DEFAULT_ABS_TOL_F32,
//         )
//     };

//     // override rel + abs
//     ($a:expr, $b:expr, $rel:expr, $abs:expr) => {
//         $crate::eq_impl_f32($a, $b, $rel, $abs)
//     };
// }

use std::ops::{Mul, Sub};

pub trait FloatX: Sized + Copy + PartialOrd + Sub<Output = Self> + Mul<Output = Self> {
    fn abs(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;

    const DEFAULT_REL_TOL: Self; // Relative diff tolerance.
    const DEFAULT_ABS_TOL: Self; // Absolute diff tolerance.

    #[inline]
    fn isclose(&self, other: &Self, rel_tol: Self, abs_tol: Self) -> bool {
        if *self == *other {
            return true;
        }

        if self.is_nan() || other.is_nan() {
            return false;
        }

        if self.is_infinite() || other.is_infinite() {
            return *self == *other;
        }

        let abs_diff = ((*self) - (*other)).abs();
        let max_tol = abs_tol.max(self.abs().max(other.abs()) * rel_tol);

        abs_diff <= max_tol
    }
}

impl FloatX for f64 {
    const DEFAULT_REL_TOL: f64 = 1e-9;
    const DEFAULT_ABS_TOL: f64 = 1e-12; // Absolute diff tolerance.

    #[inline]
    fn abs(self) -> Self {
        f64::abs(self)
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        f64::max(self, other)
    }

    #[inline]
    fn is_nan(self) -> bool {
        f64::is_nan(self)
    }

    #[inline]
    fn is_infinite(self) -> bool {
        f64::is_infinite(self)
    }
}

impl FloatX for f32 {
    const DEFAULT_REL_TOL: f32 = 1e-6;
    const DEFAULT_ABS_TOL: f32 = 1e-8;

    #[inline]
    fn abs(self) -> Self {
        f32::abs(self)
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }

    #[inline]
    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }

    #[inline]
    fn is_infinite(self) -> bool {
        f32::is_infinite(self)
    }
}

// #[macro_export]
// macro_rules! f64_isclose {
//     // 2 args → default tolerances
//     ($a:expr, $b:expr) => {{
//         use $crate::FloatX;
//         ($a as f64).isclose(
//             &($b as f64),
//             <f64 as FloatX>::DEFAULT_REL_TOL,
//             <f64 as FloatX>::DEFAULT_ABS_TOL,
//         )
//     }};

//     // 3 args → override relative tolerance
//     ($a:expr, $b:expr, $rel:expr) => {{
//         use $crate::FloatX;
//         ($a as f64).isclose(
//             &($b as f64),
//             $rel as f64,
//             <f64 as FloatX>::DEFAULT_ABS_TOL,
//         )
//     }};

//     // 4 args → override relative + absolute
//     ($a:expr, $b:expr, $rel:expr, $abs:expr) => {{
//         use $crate::floatx::FloatX;
//         ($a as f64).isclose(
//             &($b as f64),
//             $rel as f64,
//             $abs as f64,
//         )
//     }};
// }

// #[macro_export]
// macro_rules! f32_isclose {
//     // 2 args → default tolerances
//     ($a:expr, $b:expr) => {{
//         use $crate::FloatX;
//         ($a as f32).isclose(
//             &($b as f32),
//             <f32 as FloatX>::DEFAULT_REL_TOL,
//             <f32 as FloatX>::DEFAULT_ABS_TOL,
//         )
//     }};

//     // 3 args → override relative tolerance
//     ($a:expr, $b:expr, $rel:expr) => {{
//         use $crate::FloatX;
//         ($a as f32).isclose(
//             &($b as f32),
//             $rel as f32,
//             <f32 as FloatX>::DEFAULT_ABS_TOL,
//         )
//     }};

//     // 4 args → override relative + absolute
//     ($a:expr, $b:expr, $rel:expr, $abs:expr) => {{
//         use $crate::floatx::FloatX;
//         ($a as f32).isclose(
//             &($b as f32),
//             $rel as f32,
//             $abs as f32,
//         )
//     }};
// }
#[macro_export]
macro_rules! define_isclose_macro {
    ($ty:ty) => {
        paste::paste! {
            #[macro_export]
            macro_rules! [<$ty _isclose>] {
                // 2 args
                ($a:expr, $b:expr) => {{
                    use $crate::FloatX;
                    ($a as $ty).isclose(
                        &($b as $ty),
                        <$ty as FloatX>::DEFAULT_REL_TOL,
                        <$ty as FloatX>::DEFAULT_ABS_TOL,
                    )
                }};

                // 3 args
                ($a:expr, $b:expr, $rel:expr) => {{
                    use $crate::FloatX;
                    ($a as $ty).isclose(
                        &($b as $ty),
                        $rel as $ty,
                        <$ty as FloatX>::DEFAULT_ABS_TOL,
                    )
                }};

                // 4 args
                ($a:expr, $b:expr, $rel:expr, $abs:expr) => {{
                    use $crate::FloatX;
                    ($a as $ty).isclose(
                        &($b as $ty),
                        $rel as $ty,
                        $abs as $ty,
                    )
                }};
            }
        }
    };
}

define_isclose_macro!(f32);
define_isclose_macro!(f64);
