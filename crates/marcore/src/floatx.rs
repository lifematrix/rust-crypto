        pub const DEFAULT_REL_TOL_F64: f64 = 1e-9;
        pub const DEFAULT_ABS_TOL_F64: f64 = 1e-12;
        
        pub const DEFAULT_REL_TOL_F32: f32 = 1e-6;
        pub const DEFAULT_ABS_TOL_F32: f32 = 1e-8;
        
        /// Adopt the logic in Python implementation deccribed in [PEP 485](https://peps.python.org/pep-0485/)
        /// rel_tol: relative tolerance,
        /// abs_tol: absolute value tolerate
        #[inline]
        pub fn eq_impl_f64(a: f64, b: f64, rel_tol: f64, abs_tol: f64) -> bool {
            if a == b {
                return true;
            }
            
            if a.is_nan() || b.is_nan() {
                return false;
            }
            
            if a.is_infinite() || b.is_infinite() {
                return false;
            }
            
            let abs_diff = (a-b).abs();
            let max_tol = abs_tol.max(a.abs().max(b.abs())*rel_tol);
            
            abs_diff <= max_tol
        }
        
        #[inline]
        pub fn eq_impl_f32(a: f32, b: f32, rel_tol: f32, abs_tol: f32) -> bool {
            if a == b {
                return true;
            }
            
            if a.is_nan() || b.is_nan() {
                return false;
            }
            
            if a.is_infinite() || b.is_infinite() {
                return false;
            }
            
            let abs_diff = (a-b).abs();
            let max_tol = abs_tol.max(a.abs().max(b.abs())*rel_tol);
            
            abs_diff <= max_tol
        }
        
        #[macro_export]
        macro_rules! f64_eq {
            // defaults
            ($a:expr, $b:expr) => {
                $crate::eq_impl_f64(
                    $a,
                    $b,
                    $crate::floatx::DEFAULT_REL_TOL_F64,
                    $crate::floatx::DEFAULT_ABS_TOL_F64,
                )
            };
            
            // override rel only
            ($a:expr, $b:expr, $rel:expr) => {
                $crate::eq_impl_f64(
                    $a,
                    $b,
                    $rel,
                    $crate::floatx::DEFAULT_ABS_TOL_F64,
                )
            };
            
            // override rel + abs
            ($a:expr, $b:expr, $rel:expr, $abs:expr) => {
                $crate::eq_impl_f64($a, $b, $rel, $abs)
            };
        }
        
        #[macro_export]
        macro_rules! f32_eq {
            // defaults
            ($a:expr, $b:expr) => {
                $crate::eq_impl_f32(
                    $a,
                    $b,
                    $crate::floatx::DEFAULT_REL_TOL_F32,
                    $crate::floatx::DEFAULT_ABS_TOL_F32,
                )
            };
            
            // override rel only
            ($a:expr, $b:expr, $rel:expr) => {
                $crate::eq_impl_f32(
                    $a,
                    $b,
                    $rel,
                    $crate::floatx::DEFAULT_ABS_TOL_F32,
                )
            };
            
            // override rel + abs
            ($a:expr, $b:expr, $rel:expr, $abs:expr) => {
                $crate::eq_impl_f32($a, $b, $rel, $abs)
            };
        }
