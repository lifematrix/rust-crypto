use std::ops::{Add, Neg, Sub, Mul};
use std::cmp::{min, max, Ordering};
use crate::sign::MSgn;
use crate::sign::MSgn::{MPos, MNeg, MZero};



#[derive(Debug, Clone)]
pub struct MarInt {
    pub sign: MSgn,
    // little endian
    pub limbs: Vec<u64>,
}

impl MarInt {
    pub fn new() -> Self {
        Self {
            sign: MZero,
            limbs: vec![0],
        }
    }

    pub fn zero() -> Self {
        Self::new()
    }

    pub fn one() -> Self {
        Self::from_u64(1)
    }

    pub fn from_u64(value: u64) -> Self{
        Self {
            sign: MPos,
            limbs: vec![value],
        }

    }

    #[inline]
    pub(crate) fn split_u128(value: u128) -> (u64, u64) {
        let (lo, hi) = (
            (value & 0xFFFF_FFFF_FFFF_FFFF) as u64,  
            (value >> 64) as u64,                    
        );

        (lo, hi)
    }

    pub fn from_u128(value: u128) -> Self{
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

    pub fn form_i128(value: i128) -> Self {
        match value {
            0 => Self::zero(),
            _ => {
                let mut x = Self::from_u128(value.abs() as u128);
                if value < 0 {
                    x.sign = MNeg;
                }
                x
            } 
        }
    }

    
    #[inline]
    pub fn zero_limbs() -> Vec<u64> {
        vec![0 as u64]
    }

    #[inline]
    pub fn one_limbs() -> Vec<u64> {
        vec![1 as u64]
    }

    pub fn is_zero_limbs(limbs: &[u64]) -> bool {
        limbs.len() == 1 && limbs[0] == 0
    }

    pub fn is_zero(&self) -> bool {
        Self::is_zero_limbs(&self.limbs)
    }

    pub fn abs(&self) -> Self {
        if self.sign == MPos || self.sign == MZero {
            self.clone()
        }
        else {
            Self {
                sign: MPos,
                limbs: self.limbs.clone(),
            }
        }
    }

    pub fn abs_cmp(&self, other: &Self) -> Ordering {
        Self::cmp_limbs(&self.limbs, &other.limbs)
    }

    pub fn cmp_limbs(a: &Vec<u64>, b: &Vec<u64>) -> Ordering {
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

    // pub fn add_limbs(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
    //     let n = a.len().max(b.len());
    //     let mut result= Vec::with_capacity(n + 1);

    //     let mut carry: u128 = 0;
    //     for i in 0..n {
    //         let av = a.get(i).copied().unwrap_or(0) as u128;
    //         let bv = b.get(i).copied().unwrap_or(0) as u128;

    //         let sum = av + bv + carry;
    //         result.push(sum as u64);
    //         carry = sum >> 64;
    //     }

    //     if carry != 0 {
    //         result.push(carry as u64);
    //     }
    //     result
    // }

    pub fn add_limbs(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
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


    pub fn sub_limbs(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
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
                minuend += 1<<64;
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
                    minuend += 1<<64;
                    borrow = 1;
                }
                let difference = minuend - subtrahend;
                result.push(difference as u64);
            }
            else {
                result.push(a[i]);
            }
        }

        Self::trim_trailing_zero(&mut result);

        result
    }

    pub fn trim_trailing_zero(limbs: &mut Vec<u64>) {
        let mut num_trailing_zero: usize = 0;
        for i in (0..limbs.len()).rev() {
            if limbs[i] == 0 {
                num_trailing_zero += 1;
            } else {
                break;
            }
        }

        if num_trailing_zero > 0 {
            let trimmed_len = std::cmp::max(limbs.len() - num_trailing_zero, 1);
            limbs.truncate(trimmed_len);
        }
    }

    pub fn normalize(&mut self) {
        // trim most-significant zeros (end of Vec because LE)
        Self::trim_trailing_zero(&mut self.limbs);
        // canonical zero
        if self.is_zero() {
            self.sign = MZero;
        }

    }
}
