use std::io;
use std::io::{Error, ErrorKind};
use std::result::Result::Err;
use std::fs::File;

pub struct MOSEntropy;

impl MOSEntropy {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    pub fn fill_bytes(out: &mut [u8]) -> io::Result<()> {
        use std::fs::File;
        use std::io::Read;

        let mut f = File::open("/dev/urandom")?;
        f.read_exact(out)?;
        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    pub fn fill_bytes(&mut self, out: &mut [u8]) -> io::Result<()> {
        use std::env;
        Err(Error::new(
            ErrorKind::Unsupported,
            format!(
                "{} is not implemented for the target OS '{}' of family '{}' on architecture '{}'",
                std::any::type_name::<Self>().rsplit("::").next().unwrap(),
                env::consts::OS,
                env::consts::FAMILY,
                env::consts::ARCH
            ),
        ))
    }
}

impl MOSEntropy {
    pub fn next_u64() -> io::Result<u64> {
        let mut buf = [0u8; 8];
        Self::fill_bytes(&mut buf)?;
        Ok(u64::from_ne_bytes(buf))
    }

    pub fn seed256() -> io::Result<[u8; 32]> {
        let mut buf = [0u8; 32];
        Self::fill_bytes(&mut buf)?;
        Ok(buf)
    }
}