use crate::marint::MarInt;
use crate::sign::MSgn::*;
use core::str::FromStr;

const POW10: [u64; 20] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000, // 10^19
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseMarIntError {
    Empty,
    InvalidChar(char),
}

impl FromStr for MarInt {
    type Err = ParseMarIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ParseMarIntError::Empty);
        }

        // Parse sign
        let (sign, digits) = match s.as_bytes()[0] {
            b'+' => (MPos, &s[1..]),
            b'-' => (MNeg, &s[1..]),
            _ => (MPos, s),
        };

        if digits.is_empty() {
            return Err(ParseMarIntError::Empty);
        }

        // Validate (all digits)
        for ch in digits.chars() {
            if !ch.is_ascii_digit() {
                return Err(ParseMarIntError::InvalidChar(ch));
            }
        }

        // Skip leading zeros (but keep at least one digit)
        let digits = digits.trim_start_matches('0');
        if digits.is_empty() {
            return Ok(MarInt {
                sign: MZero,
                limbs: vec![0],
            });
        }

        // Parse in base 10^19 chunks
        const CHUNK: usize = 19;

        let mut limbs = MarInt::limbs_zero();
        let first_len = digits.len() % CHUNK;
        let first_len = if first_len == 0 { CHUNK } else { first_len };

        let mut pos = 0usize;
        while pos < digits.len() {
            let len = if pos == 0 { first_len } else { CHUNK };
            let part = &digits[pos..pos + len];

            let chunk_val: u64 = part.parse().unwrap(); // safe: digits-only and len<=19

            // x = x * 10^len + chunk_val
            let pow10 = POW10[len];

            limbs = MarInt::limbs_mul_by_u64(&limbs, pow10);
            limbs = MarInt::limbs_add_by_u64(&limbs, chunk_val);

            pos += len;
        }

        let mut result = MarInt { sign, limbs };
        result.normalize();
        Ok(result)
    }
}
