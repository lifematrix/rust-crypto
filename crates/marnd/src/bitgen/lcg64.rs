use std::collections::HashMap;
use std::sync::LazyLock;
use crate::MBitGen;

#[derive(Debug)]
pub struct Lcg64 {
    a: u64, 
    c: u64,
    state: u64,
}

static LCG64_PRESETS: LazyLock<HashMap<&'static str, (u64, u64)>> = LazyLock::new(|| {
    HashMap::from([
        // Donald Knuth
        ("DK", (
            0x5851_f42d_4c95_7f2d,
            0x875d_d4e5_e439_c627,             
        )),
        // Steele & Vigna
        ("SV", (
            0xd134_2543_de82_ef95,
            1,
        )),
        // PCG underlying LCG
        ("PCG64", (
            0x5851_f42d_4c95_7f2d,
            0x1405_7b7e_f767_814f,
        )),            
    ])
});

impl Lcg64 {
    pub fn new(a: u64, c: u64, seed: u64) -> Self {
        Self {a, c, state: seed}
    }

    pub fn get_preset(name: &str) -> Option<(u64, u64)> {
        LCG64_PRESETS.get(name).copied()
    }

    pub fn list_presets() -> Vec<&'static str> {
        LCG64_PRESETS.keys().copied().collect()
    }
    
    pub fn from_preset(name: &str, seed: u64) -> Option<Self> {
        let (a, c) = Self::get_preset(name)?;
        Some(Self::new(seed, a, c))
    }
}

impl MBitGen for Lcg64 {
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(self.a).wrapping_add(self.c);
        self.state
    }
}
