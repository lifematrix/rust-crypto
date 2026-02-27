use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind};
use std::result::Result::Err;
use crate::MRndErr;

pub struct MOSEntropy;

impl MOSEntropy {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    pub fn fill_bytes(out: &mut [u8]) -> Result<(), MRndErr> {
        const PATH: &str = "/dev/urandom";
        use std::fs::File;
        use std::io::Read;

        let mut f = File::open(PATH).map_err(|e| MRndErr::EntropyOpen{path: PATH, source: e})?;
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
    pub fn next_u64() -> Result<u64, MRndErr> {
        let mut buf = [0u8; 8];
        Self::fill_bytes(&mut buf)?;
        println!("MOSEnropy::next_u764 is called. return data {:?}", buf);
        Ok(u64::from_ne_bytes(buf))
    }

    pub fn seed256() -> Result<[u8; 32], MRndErr> {
        let mut buf = [0u8; 32];
        Self::fill_bytes(&mut buf)?;
        Ok(buf)
    }
}
