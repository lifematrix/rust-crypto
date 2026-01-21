use std::ops::{Sub, SubAssign};
use std::cmp::Ordering;
use crate::MarInt;

impl MarInt {
    pub fn sub_limbs(a: &[u64], b: &[u64], with_normalize: bool) -> Vec<u64> {
        debug_assert!(
            Self::cmp_limbs(a, b) != Ordering::Less,
            "sub_limbs: first operand must be >= the second one! a = {:?}, b = {:?}", a, b
        );

        let mut result = Vec::with_capacity(a.len());

        let mut borrow: u64 = 0;
        let mut minuend: u128;
        let mut subtrahend: u128;
        for i in 0..b.len() {
            minuend = a[i] as u128;
            subtrahend = b[i] as u128 + borrow as u128;
            if minuend >= subtrahend {
                borrow = 0;
            }
            else {
                minuend += Self::LIMB_BASE;
                borrow = 1;
            }
            let difference = minuend - subtrahend;
            result.push(difference as u64);
        }

        for i in b.len()..a.len() {
            if borrow != 0 {
                minuend = a[i] as u128;
                subtrahend = borrow as u128;

                if minuend >= subtrahend {
                    borrow = 0;
                }
                else {
                    minuend += Self::LIMB_BASE;
                    borrow = 1;
                }
                let difference = minuend - subtrahend;
                result.push(difference as u64);
            }
            else {
                result.push(a[i]);
            }
        }

        if with_normalize {
            Self::normalize_limbs(&mut result);
        }

        result
    }

    /// Core signed subtraction on references: a - b
    pub fn sub_ref(a: &Self, b: &Self) -> Self {
        // a - b = a + (-b)
        let mut nb = b.clone();
        nb.sign = -nb.sign;
        Self::add_ref(a, &nb)
    }
}
/* -----------------------------
 * SubAssign
 * ----------------------------- */

impl SubAssign<&MarInt> for MarInt {
    fn sub_assign(&mut self, rhs: &MarInt) {
        if rhs.is_zero() {
            return;
        }
        if self.is_zero() {
            *self = (-rhs).clone();
            return;
        }

        let out = Self::sub_ref(self, rhs);
        *self = out;
    }
}

impl SubAssign<MarInt> for MarInt {
    fn sub_assign(&mut self, rhs: MarInt) {
        *self -= &rhs;
    }
}

/* -----------------------------
 * Sub
 * ----------------------------- */

// MarInt - &MarInt
impl Sub<&MarInt> for MarInt {
    type Output = MarInt;

    fn sub(mut self, rhs: &MarInt) -> MarInt {
        self -= rhs;
        self
    }
}

// &MarInt - &MarInt
impl Sub<&MarInt> for &MarInt {
    type Output = MarInt;

    fn sub(self, rhs: &MarInt) -> MarInt {
        MarInt::sub_ref(self, rhs)
    }
}

// &MarInt - MarInt
impl Sub<MarInt> for &MarInt {
    type Output = MarInt;

    fn sub(self, rhs: MarInt) -> MarInt {
        MarInt::sub_ref(self, &rhs)
    }
}

// MarInt - MarInt
impl Sub<MarInt> for MarInt {
    type Output = MarInt;

    fn sub(self, rhs: MarInt) -> MarInt {
        self - &rhs
    }
}