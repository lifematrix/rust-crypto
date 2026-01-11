use std::ops::Neg;
use crate::marint::MarInt;
use crate::sign::MSgn::*;

/* -----------------------------
 * Neg
 * ----------------------------- */

impl Neg for MarInt {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.sign = -self.sign;
        if self.is_zero() {
            self.sign = MZero;
        }
        self
    }
}

impl Neg for &MarInt {
    type Output = MarInt;

    fn neg(self) -> MarInt {
        -self.clone()
    }
}
