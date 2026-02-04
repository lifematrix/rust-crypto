use crate::lcg::lcg64::Lcg64;
use crate::MPCfg;
use crate::mprng::MPRng;

pub struct MPBuilder;

#[derive(Debug)]
pub enum MBErr {
    MissingScheme,
    UnknownScheme(String),
    BadCfg(String), // algorithm-specific parse/validation error
}

impl std::fmt::Display for MBErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MBErr::MissingScheme => write!(f, "missing scheme"),
            MBErr::UnknownScheme(s) => write!(f, "unknown scheme '{s}'"),
            MBErr::BadCfg(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for MBErr {}

impl MPBuilder {
    pub fn build(scheme: &str, cfg: Option<MPCfg>) -> Result<Box<dyn MPRng>, MBErr> {
        let scheme = scheme.trim();
        if scheme.is_empty() {
            return Err(MBErr::MissingScheme);
        }

        let cfg = cfg.unwrap_or_default();

        match scheme.to_ascii_lowercase().as_str() {
            "lcg64" => Ok(Box::new(Lcg64::new(Some(&cfg))?)),
            // add others later
            _ => Err(MBErr::UnknownScheme(scheme.to_string())),
        }
    }
}
