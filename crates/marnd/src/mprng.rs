use crate::{CfgUtil, Lcg64, MBitGen, MOSEntropy, MPCfg, MRndErr};
use core::fmt;
use marcore::OptionExt;

#[derive(Debug)]
pub struct MPRng {
    // bitgen: Box<dyn MBitGen>,
    pub bitgen: Box<dyn MBitGen>,
    pub gen_name: String,
    pub spare_norm: Option<f64>,
}

impl MPRng {
    pub fn new(bitgen: Box<dyn MBitGen>, gen_name: &str) -> Self {
        Self {
            bitgen,
            gen_name: gen_name.into(),
            spare_norm: None,
        }
    }

    pub fn build(cfg: &MPCfg) -> Result<Self, MRndErr> {
        let schema = cfg.get_str("schema", true)?.unwrap();
        let (engine, preset) = CfgUtil::parse_schema(schema)?;

        let seed = cfg
            .get_u64("seed", false)?
            .or_try(|| MOSEntropy::next_u64())?;
        // let seed = match cfg.get_u64("seed", false)? {
        //     Some(s) => s,
        //     None => MOSEntropy::next_u64()?
        // };

        let bitgen = match engine {
            "Lcg64" => match preset {
                Some(p) => match Lcg64::from_preset(p, seed) {
                    Some(lcg) => lcg,
                    None => {
                        return Err(MRndErr::UnknownPreset {
                            engine: engine.into(),
                            preset: p.into(),
                            available: Lcg64::list_presets(),
                        });
                    }
                },
                None => {
                    let a = cfg.get_u64("a", true)?.unwrap();
                    let c = cfg.get_u64("c", true)?.unwrap();
                    Lcg64::new(a, c, seed)
                }
            },
            _ => {
                return Err(MRndErr::UnknownEngine {
                    wrong_engine: engine.into(),
                    available: vec!["Lcg64"],
                });
            }
        };
        Ok(Self::new(Box::new(bitgen), engine))
    }
}

impl fmt::Display for MPRng {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MPRng {{ gen_name: {}, bitgen: {{ {} }} }}",
            self.gen_name, self.bitgen
        )
    }
}

pub enum IntervalMode01 {
    Closed0_Open1, // [0,1)
    Open0_Closed1, // (0,1]
}

impl IntervalMode01 {
    #[inline(always)]
    fn offset(self) -> u64 {
        match self {
            IntervalMode01::Closed0_Open1 => 0,
            IntervalMode01::Open0_Closed1 => 1,
        }
    }

    #[inline(always)]
    fn fast_sign_base(self) -> (i32, i32) {
        match self {
            IntervalMode01::Closed0_Open1 => (1, -1),
            IntervalMode01::Open0_Closed1 => (-1, 2),
        }
    }
}

use std::ops::{BitOr, Shr};
use std::ops::{Add, Mul};
use std::mem;

pub trait SupportedFloat: Copy + Mul<Self, Output = Self> + Add<Self, Output = Self>  {
    type BitsType: Copy + Shr<u32, Output = Self::BitsType> + BitOr<Output = Self::BitsType>;

    const BITS_SIZE: u32 = (mem::size_of::<Self>() as u32)*8;
    const MANTISSA_DIGITS: u32;
    const SCALE: Self;

    const FAST_SHIFT_BITS: u32 = (64 - (Self::MANTISSA_DIGITS-1));
    const ONE_AS_BITS: Self::BitsType;

    fn from_u64(x: u64) -> Self;
    fn from_i32(x: i32) -> Self;
    fn from_bits(x: Self::BitsType) -> Self;
    fn u64_to_bitstype(x: u64) -> Self::BitsType;

    #[inline(always)]
    fn fast_base_from_u64(x: u64) -> Self {
        let mantissa_bits = Self::u64_to_bitstype(x >> Self::FAST_SHIFT_BITS);
        let bits = Self::ONE_AS_BITS | mantissa_bits;
        Self::from_bits(bits)
    }
}

impl SupportedFloat for f64 {
    type BitsType = u64;

    const MANTISSA_DIGITS: u32 = f64::MANTISSA_DIGITS;
    const SCALE: f64 = 1.0 / ((1u64 << Self::MANTISSA_DIGITS) as f64);
    
    // 0x3ff0_0000_0000_0000_u64
    const ONE_AS_BITS: Self::BitsType = Self::to_bits(1.0);

    #[inline(always)]
    fn from_u64(x: u64) -> Self {
        x as Self
    }

    #[inline(always)]
    fn from_i32(x: i32) -> Self {
        x as Self
    }

    // #[inline(always)]
    // fn fastbits_from_u64(x: u64) -> Self {
    //     // 0x3ff0_0000_0000_0000_u64
    //     let bits = Self::ONE_AS_BITS | (x >> Self::FAST_SHIFT_BITS);
    //     f64::from_bits(bits)
    // }

    #[inline(always)]
    fn from_bits(bits: Self::BitsType) -> Self {
        Self::from_bits(bits)
    }

    #[inline(always)]
    fn u64_to_bitstype(x: u64) -> Self::BitsType {
        x as Self::BitsType
    }

}

impl SupportedFloat for f32 {
    type BitsType = u32;

    const MANTISSA_DIGITS: u32 = f32::MANTISSA_DIGITS;
    const SCALE: f32 = 1.0 / ((1u64 << Self::MANTISSA_DIGITS) as f32);

    // 0x3f80_0000_u32
    const ONE_AS_BITS: Self::BitsType = Self::to_bits(1.0);

    #[inline(always)]
    fn from_u64(x: u64) -> Self {
        x as Self
    }

    #[inline(always)]
    fn from_i32(x: i32) -> Self {
        x as Self
    }

    // #[inline(always)]
    // fn fastbits_from_u64(x: u64) -> Self {
    //     let bits = 0x3f80_0000_u32 | ((x >> Self::FAST_SHIFT_BITS) as u32);
    //     f32::from_bits(bits)
    // }

    #[inline(always)]
    fn from_bits(bits: Self::BitsType) -> Self {
        Self::from_bits(bits)
    }

    #[inline(always)]
    fn u64_to_bitstype(x: u64) -> Self::BitsType {
        x as Self::BitsType
    }
}

impl MPRng {
    /// Produce next 64 bits of output.
    pub fn next_u64(&mut self) -> u64 {
        self.bitgen.next_u64()
    }

    /// Produce next 32 bits of output.
    #[inline(always)]
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    #[inline(always)]
    pub fn next_float_interval<T: SupportedFloat>(&mut self, mode: IntervalMode01) -> T {
        let v = (self.next_u64() >> (64 - T::MANTISSA_DIGITS)) + mode.offset();
        T::from_u64(v) * T::SCALE
    }

    /// Produce a random f64 in [0.0, 1.0).
    #[inline(always)]
    pub fn next_f64(&mut self) -> f64 {
        // const SCALE: f64 = 1.0 / ((1u64 << 53) as f64);
        // let v = self.next_u64() >> 11;
        // (v as f64) * SCALE
        self.next_float_interval(IntervalMode01::Closed0_Open1)
    }

    /// Produce a random f32 in [0.0, 1.0).
    #[inline(always)]
    pub fn next_f32(&mut self) -> f32 {
        // const SCALE: f32 = 1.0 / ((1u32 << 24) as f32);
        // let v = (self.next_u64() >> 40) as u32;
        // (v as f32) * SCALE
        // self.next_float_interval(IntervalMode01::Closed0_Open1)
        self.next_float_interval(IntervalMode01::Open0_Closed1)
    }

    #[inline(always)]
    pub fn next_float_fast_interval<T: SupportedFloat>(&mut self, mode: IntervalMode01) -> T {
        let v = self.next_u64();
        let (sign, base) = mode.fast_sign_base();
        let x = T::fast_base_from_u64(v);
        T::from_i32(sign) * x + T::from_i32(base)
    }

    pub fn next_f64_fast(&mut self) -> f64 {
        self.next_float_fast_interval(IntervalMode01::Closed0_Open1)
    }

    pub fn next_f32_fast(&mut self) -> f32 {
        self.next_float_fast_interval(IntervalMode01::Closed0_Open1)
    }

    // pub fn next_f64_fast(&mut self) -> f64 {
    //     let x = self.next_u64();
    //     // 0 01111111111 00000 ...  0000
    //     // 0011, 1111, 1111, 0000, ..., 0000
    //     let bits = 0x3ff0_0000_0000_0000_u64 | (x >> 12);

    //     f64::from_bits(bits) - 1.0
    // }

    // pub fn next_f32_fast(&mut self) -> f32 {
    //     // 0 01111111 000000 ...  0000
    //     // 0011, 1111, 1000, 0000, 0000, 0000, 0000, 0000
    //     let x = self.next_u64();
    //     let bits = 0x3f80_0000_u32 | ((x >> 41) as u32);
    //     f32::from_bits(bits) - 1.0
    // }

    /// Produce a random boolean.
    pub fn next_bool(&mut self) -> bool {
        (self.next_u64() >> 63) != 0
    }

    /// Fill a buffer with random bytes.
    pub fn fill(&mut self, out: &mut [u8]) {
        let mut i = 0;
        while i < out.len() {
            let w = self.next_u64().to_le_bytes();
            let take = (out.len() - i).min(8);
            out[i..i + take].copy_from_slice(&w[..take]);
            i += take;
        }
    }

    /// Set or reset the seed/state if the algorithm supports it.
    /// Default: do nothing.
    pub fn seed(&mut self, seed: u64) {
        self.bitgen.reseed(seed);
        self.spare_norm = None;
    }
}

impl MPRng {
    pub fn choice_idx(&mut self, probs: &[f64]) -> Result<usize, MRndErr> {
        if probs.is_empty() {
            return Ok(0);
        }

        use marcore::{f64_isclose, fmt_slice_debug};
        for (i, &p) in probs.iter().enumerate() {
            if p < 0.0 || p > 1.0 {
                return Err(MRndErr::InvalidArgument(format!(
                    "Invalid probability at index {i}: {p:.12}. Probs: {}",
                    fmt_slice_debug!(probs)
                )));
            }
        }

        let sum_p = probs.iter().sum::<f64>();
        if !f64_isclose!(sum_p, 1.0) {
            return Err(MRndErr::InvalidArgument(format!(
                "sum of probabilities must be 1.0 (got {sum_p:.12}). Probs: {}",
                fmt_slice_debug!(probs)
            )));
        }

        let mut cum_p = 0.0; // cumulative probability
        let target_p = self.next_f64();

        for (i, &p) in probs.iter().enumerate() {
            cum_p += p;
            if target_p < cum_p {
                return Ok(i);
            }
        }

        Ok(probs.len() - 1)
    }

    pub fn choice<'a, T>(&mut self, elements: &'a [T], probs: &[f64]) -> Result<&'a T, MRndErr> {
        if elements.len() != probs.len() {
            return Err(MRndErr::InvalidArgument(String::from(
                "The elements and probs must have same length",
            )));
        }

        let idx = self.choice_idx(probs)?;
        Ok(&elements[idx])
    }
}

impl MPRng {
    /// Box-Muller method
    #[inline(always)]
    pub fn norm_pair_box_muller(&mut self) -> (f64, f64) {
        // if let Some(z) = self.spare_norm.take() {
        //    return z;
        //}

        let u0: f64 = self.next_float_interval(IntervalMode01::Open0_Closed1); // (0.0, 1.0]
        let u1: f64 = self.next_f64(); // [0.0, 1.0)

        let r = (-2.0 * u0.ln()).sqrt();
        // pub const PI: f64 = 3.14159265358979323846264338327950288_f64; // 3.1415926535897931f64
        // pub const PI: f32 = 3.14159265358979323846264338327950288_f32; // 3.14159274f32
        let theta = 2.0 * std::f64::consts::PI * u1;

        let (theta_sin, theta_cos) = theta.sin_cos();
        let z0 = r * theta_cos;
        let z1 = r * theta_sin;

        //self.spare_norm = Some(z1);
        //z0
        (z0, z1)
    }

    pub fn norm_box_muller(&mut self) -> f64 {
        if let Some(z) = self.spare_norm.take() {
            return z;
        }

        let (z0, z1) = self.norm_pair_box_muller();
        self.spare_norm = Some(z1);
        z0
    }
}

impl MPRng {
    // Marsaglia(/mɑːrˈsɑːljə/) Polar Method
    #[inline(always)]
    fn norm_pair_marsaglia_polar(&mut self) -> (f64, f64) {
        loop {
            let u = 2.0 * self.next_f64() - 1.0; // [-1.0, 1.0)
            let v = 2.0 * self.next_f64() - 1.0; // [-1.0, 1.0)
            let s = u * u + v * v;

            if s > 0.0 && s < 1.0 {
                let c = (-2.0 * s.ln() / s).sqrt();
                return (u * c, v * c);
            }
        }
    }

    pub fn norm_marsaglia_polar(&mut self) -> f64 {
        if let Some(z) = self.spare_norm.take() {
            return z;
        }

        let (z0, z1) = self.norm_pair_marsaglia_polar();
        self.spare_norm = Some(z1);

        z0
    }
}
