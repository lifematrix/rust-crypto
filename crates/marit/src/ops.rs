use std::ops::{Add, Neg, Sub, Mul};
use std::cmp::{min, max, Ordering};
use crate::marint::MarInt;
use crate::sign::MSgn::*;

// // Implement add operations

impl Add for MarInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (sign, limbs) = if self.sign == rhs.sign {
            let limbs = Self::add_limbs(&self.limbs, &rhs.limbs);
            (self.sign, limbs)
        }
        else {
            match Self::cmp_limbs(&self.limbs, &rhs.limbs) {
                Ordering::Greater => {
                    let limbs = Self::sub_limbs(&self.limbs, &rhs.limbs);
                    (self.sign, limbs)
                }
                Ordering::Less => {
                    let limbs = Self::sub_limbs(&rhs.limbs, &self.limbs);
                    (rhs.sign, limbs)
                }
                Ordering::Equal => {
                    (MZero, Self::zero_limbs())
                }
            }
        };
            
        Self {
            sign,
            limbs,
        }
    }
}

impl Neg for MarInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.is_zero() {
            self
        }
        else {
            Self {
                sign: -self.sign,
                limbs: self.limbs,
            }
        }
    }
}

impl Sub for MarInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for MarInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let sign = self.sign * rhs.sign;

        //let limbs = Vec<u64>::with_capacity(self.len() + rhs.len()); 
        let mut limbs = vec![0u64; self.limbs.len() + rhs.limbs.len()]; 
        for i in 0..self.limbs.len() {
            for j in 0..rhs.limbs.len() {
                let mut idx = i + j;
                let m = self.limbs[i] as u128 * rhs.limbs[j] as u128;
                let (ml, mh) = Self::split_u128(m);
                // println!("m: {:?}", m);

                let x = limbs[idx] as u128 + ml as u128;
                let (xl, xh) = Self::split_u128(x);
                limbs[idx] = xl;

                let mut carry: u64 = mh as u64 + xh as u64;

                // println!("len of limbs: {:?}, i: {:?}, j: {:?}, carry: {:?}", limbs.len(), i, j, carry);
                while carry != 0 {
                    idx += 1;
                    let z = limbs[idx] as u128 + carry as u128;
                    let (zl, zh) = Self::split_u128(z);
                    limbs[idx] = zl;
                    carry = zh as u64;
                }
            }
        }
        Self::trim_trailing_zero(&mut limbs);

        MarInt {
            sign,
            limbs,
        }
    }
}

impl MarInt {
    pub fn from_str(s: &str) -> Self {
        let s = s.trim();
        if s.is_empty() {
            panic!("Cannot parse an empty string as MarInt");
        }

        let mut sign = MPos;
        let mut result = Self::zero();
        for (i, c) in s.chars().enumerate() {
            if i == 0 {
                if c == '-' {
                    sign = MNeg;
                    continue;
                } 
                else if c == '+' {
                    sign = MPos;
                    continue;
                }
            }
            if !c.is_ascii_digit() {
                panic!("Invalid character '{}' in input string", c);
            }

            let digit = c.to_digit(10).unwrap();
            result = result * Self::from_u64(10);
            result = result + Self::from_u64(digit as u64);
        }

        Self::trim_trailing_zero(&mut result.limbs);
        
        if Self::is_zero_limbs(&result.limbs) {
            result.sign = MZero;
        }
        else {
            result.sign = sign;
        }

        result
    }
}

impl MarInt {
    pub fn limbs_mul_by_u64(limbs: &Vec<u64>, u: u64) -> Vec<u64> {
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

        Self::trim_trailing_zero(&mut result);
    
        result
    }    
}

impl MarInt {
    pub fn divmod(self, other: Self) -> (Self, Self) {

        if other.is_zero() {
            panic!("Division by zero!");
        }

        let (q, r) = Self::divmod_limbs(&self.limbs, &other.limbs);
        
        ( 
            Self {
                sign: self.sign,
                limbs: q,
            },
            Self {
                sign: other.sign,
                limbs: r
            }
        ) 

    }

    pub fn divmod_limbs(a: &Vec<u64>, b: &Vec<u64>) -> (Vec<u64>, Vec<u64>) {
        if b.len() == 1 && b[0] == 0 {
            panic!("Division by zero!");
        }

        let ord = Self::cmp_limbs(a, b);
        if ord == Ordering::Less {
            return (Self::zero_limbs(), a.clone());
        }
        else if ord == Ordering::Equal {
            return (Self::one_limbs(), Self::zero_limbs());
        } 

        let mut dividend = a.clone();
        let mut quotient: Vec<u64> = vec![];
        let mut remainder = Self::zero_limbs();

        let mut hi: usize = a.len();
        let mut li: usize = a.len() - b.len();

        let mut stop: bool = false;

        while !stop {
            let part = &dividend[li..hi].to_vec();
            let ord = Self::cmp_limbs(part, b);
            if ord == Ordering::Less {
                if li == 0 {
                    stop = true;
                    remainder = part.clone();
                }
                else {
                    quotient.insert(0, 0);
                    li -= 1;
                    continue;
                }
            }
            else {
                let mut q_try: u64;
                if part.len() == b.len() {
                    q_try = (part[part.len()-1] as u64 / b[b.len()-1] as u64) as u64;
                } else {
                    let ph = (part[part.len()-1] as u64) << 32;
                    let pl = part[part.len()-2] as u64;
                    q_try = ((ph + pl) / b[b.len()-1] as u64) as u64; 
                }
                println!("part: {:?}, q_try: {:?}, b: {:?}", part, q_try, b);

                let mut prod_try = Self::limbs_mul_by_u64(b, q_try);
                let ord_try = Self::cmp_limbs(&part, &prod_try);
                if ord_try == Ordering::Less {
                    q_try -= 1;
                    prod_try = Self::limbs_mul_by_u64(b, q_try);
                }
                quotient.insert(0, q_try);
                let remainder_try = Self::sub_limbs(&part, &prod_try);
                for i in 0..remainder_try.len() {
                    dividend[li+i] = remainder_try[i];
                }

                for i in remainder_try.len()..part.len() {
                    dividend[li+i] = 0; 
                }
                if li == 0 {
                    stop = true;
                    remainder = remainder_try.clone();
                    continue;
                }
                hi = li + remainder_try.len(); 
                li -= 1;
            }
        }

        Self::trim_trailing_zero(&mut quotient);
        (quotient, remainder)
    }

    pub fn divmod_limbs_by_u64(limbs: &Vec<u64>, divisor: u64) -> (Vec<u64>, u64) {
        let mut quotient = Vec::with_capacity(limbs.len());
        let mut rem: u64 = 0;
    
        for &limb in limbs.iter() {
            let cur: u128 = (rem << 64) as u128 + limb as u128;
            quotient.insert(0, (cur / divisor as u128) as u64); // insert at front
            rem = ((cur as u128) % (divisor as u128)) as u64;
        }
    
        quotient.reverse();
        MarInt::trim_trailing_zero(&mut quotient);
        (quotient, rem as u64)
    }

    pub fn to_dec_str(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }

        let mut limbs = self.limbs.clone();
        let mut dec_limbs = Vec::new();

        println!("limbs: {:?}", limbs);
        while !limbs.is_empty() && !(limbs.len() == 1 && limbs[0] == 0) {
            let (q, r) = Self::divmod_limbs_by_u64(&limbs, 10);
            println!("q, r: {:?}, {:?}", q, r);
            limbs = q;
            dec_limbs.push((r as u8) + b'0');
        }

        dec_limbs.reverse();

        let mut result = String::from_utf8(dec_limbs).unwrap();

        if self.sign == MNeg {
            result = format!("-{}", result);
        }

        result
    }
}
