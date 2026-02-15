use crate::sign::MSgn;
use crate::sign::MSgn::{MNeg, MPos, MZero};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct MarInt {
    pub sign: MSgn,
    // little endian
    pub limbs: Vec<u64>,
}

impl MarInt {
    pub const LIMB_BITS: u32 = 64;
    pub const LIMB_BASE: u128 = 1u128 << Self::LIMB_BITS;
    pub const LIMB_MASK: u128 = Self::LIMB_BASE - 1;
}

impl MarInt {
    pub fn new() -> Self {
        Self {
            sign: MZero,
            limbs: Self::limbs_zero(),
        }
    }

    pub fn zero() -> Self {
        Self::new()
    }

    pub fn one() -> Self {
        Self::from_u64(1)
    }

    pub fn from_u64(value: u64) -> Self {
        if value == 0 {
            return Self::zero();
        }
        Self {
            sign: MPos,
            limbs: vec![value],
        }
    }

    #[inline]
    pub(crate) fn split_u128(value: u128) -> (u64, u64) {
        let (lo, hi) = ((value & 0xFFFF_FFFF_FFFF_FFFF) as u64, (value >> 64) as u64);

        (lo, hi)
    }

    pub fn from_u128(value: u128) -> Self {
        let (low, high) = Self::split_u128(value);
        if value == 0 {
            return Self::zero();
        }

        let limbs = if high != 0 {
            vec![low, high]
        } else {
            vec![low]
        };

        Self {
            sign: MPos,
            limbs: limbs,
        }
    }

    pub fn from_i128(value: i128) -> Self {
        match value {
            0 => Self::zero(),
            i128::MIN => {
                let mut x = Self::from_u128(1u128 << 127);
                x.sign = MNeg;
                x
            }
            _ => {
                // safe because i128::MIN handled above
                let mut x = Self::from_u128(value.abs() as u128);
                if value < 0 {
                    x.sign = MNeg;
                }
                x
            }
        }
    }

    #[inline]
    pub fn limbs_zero() -> Vec<u64> {
        vec![0]
    }

    #[inline]
    pub fn limbs_one() -> Vec<u64> {
        vec![1]
    }

    pub fn is_limbs_zero(limbs: &[u64]) -> bool {
        limbs.len() == 1 && limbs[0] == 0
    }

    pub fn is_limbs_one(limbs: &[u64]) -> bool {
        limbs.len() == 1 && limbs[0] == 1
    }

    pub fn is_zero(&self) -> bool {
        self.sign == MZero || Self::is_limbs_zero(&self.limbs)
    }

    pub fn abs(&self) -> Self {
        if self.sign == MPos || self.sign == MZero {
            self.clone()
        } else {
            Self {
                sign: MPos,
                limbs: self.limbs.clone(),
            }
        }
    }

    pub fn abs_cmp(&self, other: &Self) -> Ordering {
        Self::cmp_limbs(&self.limbs, &other.limbs)
    }

    pub fn cmp_limbs(a: &[u64], b: &[u64]) -> Ordering {
        if a.len() != b.len() {
            return a.len().cmp(&b.len());
        }

        for i in (0..a.len()).rev() {
            let ord = a[i].cmp(&b[i]);
            if ord != Ordering::Equal {
                return ord;
            }
        }

        Ordering::Equal
    }

    pub fn normalize_limbs(limbs: &mut Vec<u64>) {
        // let mut num_trailing_zero: usize = 0;
        // for i in (0..limbs.len()).rev() {
        //     if limbs[i] == 0 {
        //         num_trailing_zero += 1;
        //     } else {
        //         break;
        //     }
        // }

        // if num_trailing_zero > 0 {
        //     let trimmed_len = std::cmp::max(limbs.len() - num_trailing_zero, 1);
        //     limbs.truncate(trimmed_len);
        // }

        if limbs.is_empty() {
            limbs.push(0);
            return;
        }

        while limbs.len() > 1 && *limbs.last().unwrap() == 0 {
            limbs.pop();
        }
    }

    pub fn normalize(&mut self) {
        // trim most-significant zeros (end of Vec because LE)
        Self::normalize_limbs(&mut self.limbs);
        // canonical zero
        if self.is_zero() {
            self.sign = MZero;
        }
    }
}
