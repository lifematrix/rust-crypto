use std::ops::{Add, AddAssign};
use std::cmp::Ordering;
use crate::MarInt;
use crate::sign::MSgn::*;

impl MarInt {
    pub fn limbs_add_by_u64(limbs: &[u64], u: u64) -> Vec<u64> {
        if u == 0 {
            return limbs.to_vec();
        }

        let mut result = limbs.to_vec();
        let mut carry: u64 = u;
        let mut i = 0usize;

        while carry != 0 {
            if i == result.len() {
                result.push(0);
            }

            let m = (result[i] as u128) + carry as u128;
            let (ml, mh) = Self::split_u128(m);
            result[i] = ml;
            carry = mh;
            i += 1;
        }

        result
    }

    pub fn add_limbs(a: &[u64], b: &[u64]) -> Vec<u64> {
        let mut result = Vec::with_capacity(a.len().max(b.len()) + 1);

        let (comm_len, longer_one) = if a.len() > b.len() {
            (b.len(), a)
        } else {
            (a.len(), b)
        };

        let mut carry: u64 = 0;
        for i in 0..comm_len {
            let x: u128 = a[i] as u128 + b[i] as u128 + carry as u128;
            let (low, high) = Self::split_u128(x);          
            result.push(low);
            carry = high;
        }

        for i in comm_len..longer_one.len() {
            if carry != 0 {
                let x: u128 = longer_one[i] as u128 + carry as u128;
                let (low, high) = Self::split_u128(x);
                result.push(low);
                carry = high;    
            }
            else {
                result.push(longer_one[i]);
            }
        }

        if carry != 0 {
            result.push(carry);
        }
        
        result
    }

    /// Core signed add on references, returning a fresh normalized result.
    /// Single source of truth for addition.
    pub fn add_ref(a: &Self, b: &Self) -> Self {
        // Fast paths using your existing zero logic
        if a.is_zero() {
            return b.clone();
        }
        if b.is_zero() {
            return a.clone();
        }

        let (sign, limbs) = if a.sign == b.sign {
            (a.sign, Self::add_limbs(&a.limbs, &b.limbs))
        } else {
            match Self::cmp_limbs(&a.limbs, &b.limbs) {
                Ordering::Greater => (a.sign, Self::sub_limbs(&a.limbs, &b.limbs, true)),
                Ordering::Less => (b.sign, Self::sub_limbs(&b.limbs, &a.limbs, true)),
                Ordering::Equal => (MZero, Self::limbs_zero()),
            }
        };

        let mut out = Self { sign, limbs };
        out.normalize();
        out
    }
}

/* -----------------------------
 * AddAssign
 * ----------------------------- */

impl AddAssign<&MarInt> for MarInt {
    fn add_assign(&mut self, rhs: &MarInt) {
        if rhs.is_zero() {
            return;
        }
        if self.is_zero() {
            *self = rhs.clone();
            return;
        }

        let out = Self::add_ref(self, rhs);
        *self = out;
    }
}

impl AddAssign<MarInt> for MarInt {
    fn add_assign(&mut self, rhs: MarInt) {
        *self += &rhs;
    }
}

/* -----------------------------
 * Add
 * ----------------------------- */

// MarInt + &MarInt
impl Add<&MarInt> for MarInt {
    type Output = MarInt;

    fn add(mut self, rhs: &MarInt) -> MarInt {
        self += rhs;
        self
    }
}

// &MarInt + &MarInt
impl Add<&MarInt> for &MarInt {
    type Output = MarInt;

    fn add(self, rhs: &MarInt) -> MarInt {
        MarInt::add_ref(self, rhs)
    }
}

// &MarInt + MarInt
impl Add<MarInt> for &MarInt {
    type Output = MarInt;

    fn add(self, rhs: MarInt) -> MarInt {
        MarInt::add_ref(self, &rhs)
    }
}

// MarInt + MarInt
impl Add<MarInt> for MarInt {
    type Output = MarInt;

    fn add(self, rhs: MarInt) -> MarInt {
        self + &rhs
    }
}