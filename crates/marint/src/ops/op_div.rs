use crate::MSgn::*;
use crate::MarInt;
use std::cmp::{Ordering, max, min};

impl MarInt {
    pub fn shortdiv_limbs_by_u64(limbs: &[u64], divisor: u64) -> (Vec<u64>, u64) {
        let mut rem: u128 = 0;

        if divisor == 0 {
            panic!("Division by zero");
        }

        if Self::is_limbs_zero(limbs) {
            return (Self::limbs_zero(), 0);
        }

        if divisor == 1 {
            let mut q = limbs.to_vec();
            Self::normalize_limbs(&mut q);
            return (q, 0);
        }

        let mut quotient = vec![0u64; limbs.len()];
        let d = divisor as u128;

        for i in (0..limbs.len()).rev() {
            let cur: u128 = (rem << Self::LIMB_BITS) + limbs[i] as u128;
            quotient[i] = (cur / d) as u64;
            rem = cur % d;
        }

        Self::normalize_limbs(&mut quotient);
        (quotient, rem as u64)
    }

    /// a: limbs of dividend
    /// b: limbs of divisor
    ///
    pub fn longdiv_limbs(a: &[u64], b: &[u64]) -> (Vec<u64>, Vec<u64>) {
        if Self::is_limbs_zero(b) {
            // the divisor is 0
            panic!("Division by Zero");
        }

        if b.len() == 1 {
            // short div is enough.
            let (short_q, short_r) = MarInt::shortdiv_limbs_by_u64(a, b[0]);
            return (short_q, vec![short_r]);
        }

        if Self::is_limbs_one(&b) {
            // the divisor is 1
            return (a.to_vec(), Self::limbs_zero());
        }

        let cmp_flag = Self::cmp_limbs(a, b);
        if cmp_flag == Ordering::Less {
            // if a < b, quotient = 0, remainder = a
            return (Self::limbs_zero(), a.to_vec());
        } else if cmp_flag == Ordering::Equal {
            // if a == b, quotient = 1, remainder = 0
            return (Self::limbs_one(), Self::limbs_zero());
        }

        let n = b.len();
        let m = a.len() - n;

        let dnorm: u64 = (Self::LIMB_BASE / (b[n - 1] as u128 + 1)) as u64;
        let mut a = Self::limbs_mul_by_u64(a, dnorm);
        let mut v = Self::limbs_mul_by_u64(b, dnorm);

        if a.len() < m + n + 1 {
            a.push(0);
        }

        v.push(0);
        let b = &v[0..v.len() - 1];

        let mut quotient: Vec<u64> = vec![0; m + 1];
        let mut remainder: Vec<u64> = vec![0; n + 1];

        for j in (0..=m).rev() {
            let dv = (a[j + n] as u128) * Self::LIMB_BASE + a[j + n - 1] as u128;
            let mut q = dv / b[n - 1] as u128;
            let mut r = dv % b[n - 1] as u128;
            while q >= Self::LIMB_BASE
                || q * (b[n - 2] as u128) > (Self::LIMB_BASE * r + (a[j + n - 2] as u128))
            {
                q -= 1;
                r += b[n - 1] as u128;

                if r >= Self::LIMB_BASE {
                    break;
                }
            }

            let u = &mut a[j..j + n + 1];

            loop {
                let qv = Self::limbs_mul_by_u64(&v, q as u64);
                if Self::cmp_limbs(&u, &qv) == Ordering::Less {
                    q -= 1;
                    continue;
                } else {
                    let u_left = Self::sub_limbs(&u, &qv, false);
                    for k in 0..n + 1 {
                        u[k] = u_left[k];
                    }
                    if j == 0 {
                        remainder = u_left;
                    }
                    break;
                }
            }
            quotient[j] = q as u64;
            // println!("j = {}, q = {}", j, q);
        }

        // remove the trailing zeros of quotient firstly.
        Self::normalize_limbs(&mut quotient);
        Self::normalize_limbs(&mut remainder);

        // Unnormalize the remainder.
        let (mut remainder, r_remainder) = Self::shortdiv_limbs_by_u64(&remainder, dnorm);
        assert_eq!(r_remainder, 0, "r_remainder{} = 0", r_remainder);
        Self::normalize_limbs(&mut remainder);

        return (quotient, remainder);
    }

    fn div_rem(&self, rhs: &MarInt) -> (MarInt, MarInt) {
        if rhs.is_zero() {
            panic!("Division by zero");
        }

        if self.is_zero() {
            return (MarInt::zero(), MarInt::zero());
        }

        let (q_limbs, r_limbs) = MarInt::longdiv_limbs(&self.limbs, &rhs.limbs);

        let mut q = MarInt {
            sign: self.sign * rhs.sign,
            limbs: q_limbs,
        };

        let mut r = MarInt {
            sign: self.sign,
            limbs: r_limbs,
        };

        q.normalize();
        r.normalize();

        (q, r)
    }

    /// Euclidean division: remainder is always non-negative.
    /// Adjusts the truncating quotient/remainder when remainder is negative.
    pub fn div_euclid(&self, rhs: &MarInt) -> MarInt {
        let (mut q, mut r) = self.div_rem(rhs);
        if r.sign == MNeg {
            // r += |rhs|
            r += rhs.abs();
            // q -= sign(rhs)
            let mut adj = MarInt::one();
            adj.sign = rhs.sign;
            q -= &adj;
        }
        q
    }

    /// Euclidean remainder: always in [0, |rhs|).
    pub fn rem_euclid(&self, rhs: &MarInt) -> MarInt {
        let (mut q, mut r) = self.div_rem(rhs);
        if r.sign == MNeg {
            r += rhs.abs();
            let mut adj = MarInt::one();
            adj.sign = rhs.sign;
            q -= &adj;
        }
        r
    }
}

use std::ops::{Div, Rem};
impl Div<&MarInt> for &MarInt {
    type Output = MarInt;

    fn div(self, rhs: &MarInt) -> MarInt {
        self.div_rem(rhs).0
    }
}

impl Rem<&MarInt> for &MarInt {
    type Output = MarInt;

    fn rem(self, rhs: &MarInt) -> MarInt {
        self.div_rem(rhs).1
    }
}
