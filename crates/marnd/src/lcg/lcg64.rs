use crate::mprng::MPRng;
use crate::MPCfg;

pub struct Lcg64 {
    state: u64,
    a: u64,
    c: u64,
}

impl Lcg64 {
    const A_A: u64 = 6364136223846793005;
    const C_A: u64 = 1442695040888963407;

    const A_B: u64 = 2862933555777941757;
    const C_B: u64 = 3037000493;

    pub fn new(cfg: Option<&MPCfg>) -> Result<Self, crate::MBErr> {
        Self::from_variant(None, cfg)
    }

    pub fn from_variant(variant: Option<&str>, cfg: Option<&MPCfg>) -> Result<Self, crate::MBErr> {
        let (default_a, default_c) = Self::constants_for_variant(variant)?;

        let (a, c, state) = match cfg {
            Some(g) => (
                g.get_u64_or("a", default_a)?,
                g.get_u64_or("c", default_c)?,
                g.get_u64_or("seed", 0)?,
            ),
            None => (default_a, default_c, Self::seed_from_time_u64()),
        };

        Ok(Self { state, a, c })
    }

    fn constants_for_variant(variant: Option<&str>) -> Result<(u64, u64), crate::MBErr> {
        match variant.map(|s| s.trim()).filter(|s| !s.is_empty()) {
            None => Ok((Self::A_A, Self::C_A)),
            Some(v) if v.eq_ignore_ascii_case("a") => Ok((Self::A_A, Self::C_A)),
            Some(v) if v.eq_ignore_ascii_case("b") => Ok((Self::A_B, Self::C_B)),
            Some(v) => Err(crate::MBErr::UnknownVariant {
                family: "Lcg64".to_string(),
                variant: v.to_string(),
            }),
        }
    }
}

impl MPRng for Lcg64 {
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(self.a).wrapping_add(self.c);
        self.state
    }

    fn seed(&mut self, seed: u64) {
        self.state = seed;
    }
}
