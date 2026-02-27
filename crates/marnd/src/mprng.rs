use core::fmt;
use std::fmt::Formatter;
use crate::{CfgUtil, Lcg64, MBitGen, MOSEntropy, MPCfg, MRndErr};
use marcore::OptionExt;

#[derive(Debug)]
pub struct MPRng {
    // bitgen: Box<dyn MBitGen>,
    pub bitgen: Box<dyn MBitGen>,
    pub gen_name: String,
}

impl MPRng {
    pub fn new(bitgen: Box<dyn MBitGen>, gen_name: &str) -> Self {
        Self { bitgen, gen_name: gen_name.into() }
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
                        None => return Err(MRndErr::UnknownPreset {
                                engine: engine.into(),
                                preset: p.into(),
                                available: Lcg64::list_presets() })
                        },
                None => {
                    let a = cfg.get_u64("a", true)?.unwrap();
                    let c = cfg.get_u64("c", true)?.unwrap();
                    Lcg64::new(a, c, seed)
                }
            },
            _ => return Err(MRndErr::UnknownEngine{
                wrong_engine: engine.into(),
                available: vec!["Lcg64"],
            })
        };
        Ok(Self::new(Box::new(bitgen), engine))
    }
}

impl fmt::Display for MPRng {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MPRng {{ gen_name: {}, bitgen: {{ {} }} }}", self.gen_name, self.bitgen)
    }
}

impl MPRng {
    /// Produce next 64 bits of output.
    pub fn next_u64(&mut self) -> u64 {
        self.bitgen.next_u64()
    }

    /// Produce next 32 bits of output.
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Produce a random f64 in [0.0, 1.0).
    pub fn next_f64(&mut self) -> f64 {
        const SCALE: f64 = 1.0 / ((1u64 << 53) as f64);
        let v = self.next_u64() >> 11;
        (v as f64) * SCALE
    }

    /// Produce a random f32 in [0.0, 1.0).
    pub fn next_f32(&mut self) -> f32 {
        const SCALE: f32 = 1.0 / ((1u32 << 24) as f32);
        let v = (self.next_u64() >> 40) as u32;
        (v as f32) * SCALE
    }

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
    }
}

impl MPRng {
    pub fn choice_idx(&mut self, probs: &[f64]) -> usize {
        if probs.is_empty() {
            return 0;
        }

        #[cfg(debug_assertions)]
        {
            use marcore::{f64_isclose, fmt_slice_debug};

            for (i, &p) in probs.iter().enumerate() {
                if p < 0.0 || p > 1.0 {
                    panic!(
                        "Invalid probability at index {i}: {p:.12}. Probs: {}",
                        fmt_slice_debug!(probs)
                    );
                }
            }

            let sum_p = probs.iter().sum::<f64>();
            if !f64_isclose!(sum_p, 1.0) {
                panic!(
                    "sum of probabilities must be 1.0 (got {sum_p:.12}). Probs: {}",
                    fmt_slice_debug!(probs)
                );
            }
        }

        let mut cum_p = 0.0; // cumulative probability
        let target_p = self.next_f64();

        for (i, &p) in probs.iter().enumerate() {
            cum_p += p;
            if target_p < cum_p {
                return i;
            }
        }
        return probs.len() - 1;
    }

    pub fn choice<'a, T>(&mut self, elements: &'a [T], probs: &[f64]) -> &'a T {
        assert_eq!(
            elements.len(),
            probs.len(),
            "The elements and probs must have same length"
        );
        &elements[self.choice_idx(probs)]
    }
}
