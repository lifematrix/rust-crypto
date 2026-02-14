use std::io;
use std::result::Result::Err;
use std::io::{Error, ErrorKind};

pub trait MOSEntropy {
    fn fill_bytes(&mut self, out: &mut [u8]) -> io::Result<()>;

    fn next_u64(&mut self) -> io::Result<u64> {
        let mut buf = [0u8; 8];
        self.fill_bytes(&mut buf)?;
        Ok(u64::from_ne_bytes(buf))
    }

    fn seed256(&mut self) -> io::Result<[u8; 32]> {
        let mut buf = [0u8; 32];
        self.fill_bytes(&mut buf)?;
        Ok(buf)
    }
}

 pub struct MOSRng;
// pub struct MOSRng {
//     #[cfg(any(target_os = "macos", target_os = "linux"))]
//     handle: File,
// }

//#[cfg(any(target_os = "macos", target_os = "linux"))]
#[cfg(any(target_os = "macos"))]
impl MOSEntropy for MOSRng {
    fn fill_bytes(&mut self, out: &mut [u8]) -> io::Result<()> {
        use std::fs::File;
        use std::io::Read;

        let mut f = File::open("/dev/urandom")?;
        f.read_exact(out)?;
        Ok(())
    } 
}

#[cfg(not(any(target_os = "macos")))]
impl MOSEntropy for MOSRng {
    fn fill_bytes(&mut self, out: &mut [u8]) -> io::Result<()> {
        use std::env;
        Err(Error::new(
            ErrorKind::Unsupported, 
            format!("{} is not implemented for the target OS '{}' of family '{}' on architecture '{}'",
                std::any::type_name::<Self>()
                    .rsplit("::")
                    .next()
                    .unwrap(),
                env::consts::OS,
                env::consts::FAMILY,
                env::consts::ARCH)
        ))
    }
}