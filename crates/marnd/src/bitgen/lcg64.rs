use crate::MBitGen;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct Lcg64 {
    pub a: u64,
    pub c: u64,
    pub state: u64,
    pub preset: Option<String>,
}

static LCG64_PRESETS: LazyLock<HashMap<&'static str, (u64, u64)>> = LazyLock::new(|| {
    HashMap::from([
        // Donald Knuth
        ("DK", (0x5851_f42d_4c95_7f2d, 0x875d_d4e5_e439_c627)),
        // Steele & Vigna
        ("SV", (0xd134_2543_de82_ef95, 1)),
        // PCG underlying LCG
        ("PCG64", (0x5851_f42d_4c95_7f2d, 0x1405_7b7e_f767_814f)),
    ])
});

impl Lcg64 {
    pub fn new(a: u64, c: u64, seed: u64) -> Self {
        Self { a, c, state: seed, preset: None }
    }

    pub fn get_preset(name: &str) -> Option<(u64, u64)> {
        LCG64_PRESETS.get(name).copied()
    }

    pub fn list_presets() -> Vec<&'static str> {
        let mut v = LCG64_PRESETS.keys().copied().collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    pub fn from_preset(name: &str, seed: u64) -> Option<Self> {
        let (a, c) = Self::get_preset(name)?;
        let mut lcg = Self::new(a, c, seed);
        lcg.preset = Some(name.into());
        Some(lcg)
    }

    /// Fast path for direct (non-trait-object) benchmarks and callers.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(self.a).wrapping_add(self.c);
        self.state
    }
}

impl MBitGen for Lcg64 {
    #[inline]
    fn next_u64(&mut self) -> u64 {
        Lcg64::next_u64(self)
    }

    fn reseed(&mut self, seed: u64) {
        self.state = seed;
    }
}

use core::fmt;

fn fmt_u64_dual(f: &mut fmt::Formatter<'_>, name: &str, v: u64) -> fmt::Result {
    write!(f, "{}: {} (0x{:016x})", name, v, v)
}

impl fmt::Display for Lcg64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use core::any::type_name;
        let self_type_name = type_name::<Self>();
        // write!(f, "Lcg64 {{ ")?;
        write!(f, "struct type: '{}' {{ ", self_type_name)?;
        write!(f, "preset: {:?}", self.preset)?;
        write!(f, ", ")?;
        fmt_u64_dual(f, "a", self.a)?;
        write!(f, ", ")?;
        fmt_u64_dual(f, "c", self.c)?;
        write!(f, ", ")?;
        fmt_u64_dual(f, "state", self.state)?;
        write!(f, " }}")
    }
}
