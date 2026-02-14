use std::time::{SystemTime, UNIX_EPOCH};

pub trait MPRng {
    /// Produce next 64 bits of output.
    fn next_u64(&mut self) -> u64;

    /// Produce next 32 bits of output.
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Produce a random f64 in [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        const SCALE: f64 = 1.0 / ((1u64 << 53) as f64);
        let v = self.next_u64() >> 11;
        (v as f64) * SCALE
    }

    /// Produce a random f32 in [0.0, 1.0).
    fn next_f32(&mut self) -> f32 {
        const SCALE: f32 = 1.0 / ((1u32 << 24) as f32);
        let v = (self.next_u64() >> 40) as u32;
        (v as f32) * SCALE
    }

    /// Produce a random boolean.
    fn next_bool(&mut self) -> bool {
        (self.next_u64() >> 63) != 0
    }


    // fn choice<'aa, T>(&mut self, elements: &'aa [T], probs: &[f64]) -> &'aa T {
    //     let idx = self.choice_idx(probs);
    //     &elements[idx]
    // }

    /// Fill a buffer with random bytes.
    fn fill(&mut self, out: &mut [u8]) {
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
    fn seed(&mut self, _seed: u64) {
        // not all RNGs support reseeding
    }

    fn seed_from_time_u64() -> u64
    where
        Self: Sized,
    {
        let dur = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        // Mix seconds and nanos, then truncate to u64.
        let nanos = dur.subsec_nanos() as u64;
        dur.as_secs().wrapping_shl(32) ^ nanos
    }
}

pub trait MPRngExt: MPRng {
    fn choice_idx(&mut self, probs: &[f64]) -> usize {
        if probs.is_empty() {
            return 0; 
        }

        #[cfg(debug_assertions)]
        {
            use marcore::{f64_isclose, fmt_slice_debug};

            for (i, &p) in probs.iter().enumerate() {
                if p < 0.0 || p > 1.0 {
                    panic!("Invalid probability at index {i}: {p:.12}. Probs: {}", fmt_slice_debug!(probs));
                } 
            }

            let sum_p = probs.iter().sum::<f64>();
            if !f64_isclose!(sum_p, 1.0) {
                panic!("sum of probabilities must be 1.0 (got {sum_p:.12}). Probs: {}", fmt_slice_debug!(probs));
            }
        }

        let mut cum_p = 0.0;   // cumulative probability
        let target_p = self.next_f64();  

        for (i, &p) in probs.iter().enumerate() {
            cum_p += p;
            if cum_p > target_p {
                return i;
            }
        }
        return probs.len() - 1;
    }

    fn choice<'a, T>(&mut self, elements: &'a [T], probs: &[f64]) -> &'a T {
        assert_eq!(
            elements.len(),
            probs.len(),
            "The elements and probs must have same length"
        );
        &elements[self.choice_idx(probs)]
    }
}

// This makes *every* MPRng implement MPRngExt automatically (including Lcg64).
impl<R: MPRng + ?Sized> MPRngExt for R {}
