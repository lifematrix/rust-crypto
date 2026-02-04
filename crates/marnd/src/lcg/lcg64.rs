use crate::MPCfg;
use crate::mprng::MPRng;

pub struct Lcg64 {
    state: u64,
    a: u64,
    c: u64
}

impl Lcg64 {
    const DEFAULT_A: u64 = 6364136223846793005;
    const DEFAULT_C: u64 = 1442695040888963407;

    pub fn new(cfg: Option<&MPCfg>) -> Result<Self, crate::MBErr> {
        let (a, c, state) = match cfg {
            Some(g) => (
                g.get_u64_or("a", Self::DEFAULT_A)?,
                g.get_u64_or("c", Self::DEFAULT_C)?,
                g.get_u64_or("seed", 0)?,
            ),
            None => (Self::DEFAULT_A, Self::DEFAULT_C, Self::seed_from_time_u64()),
        };

        Ok(Self { state, a, c })
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
