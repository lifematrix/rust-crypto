use std::ops::{Mul, MulAssign};
use std::cmp::Ordering;
use crate::MarInt;
use crate::MSgn::*;

impl MarInt {
    /// Multiply magnitudes (little-endian u64 limbs), returning magnitude limbs.
    /// `a` and `b` must be normalized magnitudes (but can be [0]).
    pub fn mul_limbs_ref(a: &[u64], b: &[u64]) -> Vec<u64> {
        // If either is zero => zero
        if Self::is_limbs_zero(a) || Self::is_limbs_zero(b) {
            return Self::limbs_zero();
        }

        // +1 limb for safety on carry propagation
        let mut limbs = vec![0u64; a.len() + b.len() + 1];

        for i in 0..a.len() {
            for j in 0..b.len() {
                let mut idx = i + j;

                let m = a[i] as u128 * b[j] as u128;
                let (ml, mh) = Self::split_u128(m);

                // add low part into limbs[idx]
                let x = limbs[idx] as u128 + ml as u128;
                let (xl, xh) = Self::split_u128(x);
                limbs[idx] = xl;

                // carry = high(m) + carry-from-low-add
                let mut carry: u64 = mh.wrapping_add(xh);

                while carry != 0 {
                    idx += 1;
                    let z = limbs[idx] as u128 + carry as u128;
                    let (zl, zh) = Self::split_u128(z);
                    limbs[idx] = zl;
                    carry = zh;
                }
            }
        }

        // Trim to canonical form for magnitudes ([0] if all zero)
        Self::normalize_limbs(&mut limbs);
        limbs
    }

    /// Core signed multiplication on references.
    pub fn mul_ref(a: &Self, b: &Self) -> Self {
        // Fast paths
        if a.is_zero() || b.is_zero() {
            return Self::zero();
        }

        let sign = a.sign * b.sign;
        let limbs = Self::mul_limbs_ref(&a.limbs, &b.limbs);

        let mut out = Self { sign, limbs };
        out.normalize();
        out
    }
}

/* -----------------------------
 * MulAssign
 * ----------------------------- */

impl MulAssign<&MarInt> for MarInt {
    fn mul_assign(&mut self, rhs: &MarInt) {
        if self.is_zero() || rhs.is_zero() {
            *self = Self::zero();
            return;
        }

        let out = Self::mul_ref(self, rhs);
        *self = out;
    }
}

impl MulAssign<MarInt> for MarInt {
    fn mul_assign(&mut self, rhs: MarInt) {
        *self *= &rhs;
    }
}

/* -----------------------------
 * Mul
 * ----------------------------- */

// MarInt * &MarInt  (move left, borrow right)
impl Mul<&MarInt> for MarInt {
    type Output = MarInt;

    fn mul(self, rhs: &MarInt) -> MarInt {
        // compute from refs; `self` is owned so taking &self is fine
        Self::mul_ref(&self, rhs)
    }
}

// &MarInt * &MarInt  (borrow both)
impl Mul<&MarInt> for &MarInt {
    type Output = MarInt;

    fn mul(self, rhs: &MarInt) -> MarInt {
        MarInt::mul_ref(self, rhs)
    }
}

// &MarInt * MarInt  (borrow left, move right)
impl Mul<MarInt> for &MarInt {
    type Output = MarInt;

    fn mul(self, rhs: MarInt) -> MarInt {
        MarInt::mul_ref(self, &rhs)
    }
}

// MarInt * MarInt (move both)
impl Mul<MarInt> for MarInt {
    type Output = MarInt;

    fn mul(self, rhs: MarInt) -> MarInt {
        self * &rhs
    }
}

impl MarInt {
    pub fn limbs_mul_by_u64(limbs: &[u64], u: u64) -> Vec<u64> {
        let mut result = Vec::with_capacity(limbs.len() + 1);
        let mut carry: u64 = 0;
    
        for &digit in limbs.iter() {
            let m = digit as u128 * u as u128 + carry as u128; // Multiply and add carry
            let (ml, mh) = Self::split_u128(m);
            result.push(ml); // Push lower part
            carry = mh as u64; // Carry is the upper part
        }
    
        if carry != 0 {
            result.push(carry as u64);
        }

        Self::normalize_limbs(&mut result);
    
        result
    }    
}
