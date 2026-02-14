use std::io;
use std::io::{Err, ErrorKind};

pub trait MOSEntropy {
    fn fill_bytes(&mut self, out: &mut [u8]) -> io::Result<()> {
        use std::env;
        Err(Err::new(
            ErrorKind::Unsupported, 
            "{} is not implemented for the target OS '{}' of family '{}' on architecture '{}'",
            env::consts::OS,
            env::consts::FAMILY,
            env::consts::ARCH
        ))
    }

    fn next_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; 8];
        self.fill_bytes(&mut b)?;
        Ok(u64::from_ne_bytes(buf))
    }

    fn seed256(&mut self) -> io::Result<[u8; 32]> {
        let mut buf = [0u8; 32];
        self.fill_bytes(&mut buf)?;
        Ok(buf)
    }
}

// pub struct MOSRng {
//     #[cfg(any(target_os = "macos", target_os = "linux"))]
//     handle: File,
// }

#[cfg(any(target_os = "macos", target_os = "linux"))]
impl MOSEntropy for MOSRng {
    fn fill_bytes(&mut self, out: &mut [u8]) -> io::Result<()> {
        use std::fs::File;
        use std::io::Read;

        let mut f = File::open("/dev/urandom")?;
        f.read_exact(out)?;
        Ok(())
    } 
}