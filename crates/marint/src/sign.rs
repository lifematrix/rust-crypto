
#[repr(i8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MSgn {
    MNeg = -1,
    MZero = 0,
    MPos = 1,
}

use MSgn::*;

impl core::ops::Neg for MSgn {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            MPos => MNeg,
            MNeg => MPos,
            MZero => MZero,
        }
    }
}


impl core::ops::Mul for MSgn {
    type Output = MSgn;

    #[inline]
    fn mul(self, rhs: MSgn) -> MSgn {
        match (self, rhs) {
            (MZero, _) | (_, MZero) => MZero,
            (MPos, MPos) | (MNeg, MNeg) => MPos,
            (MPos, MNeg) | (MNeg, MPos) => MNeg,
        }
    }
}