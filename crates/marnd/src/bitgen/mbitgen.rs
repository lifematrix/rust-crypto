use core::fmt;

pub trait MBitGen: Send + Sync + fmt::Debug {
    fn next_u64(&mut self) -> u64;

    fn fill_u64(&mut self, out: &mut [u64]) {
        for x in out {
            *x = self.next_u64();
        }
    }
}


