use crate::lcg::lcg64::Lcg64;
use crate::mprng::MPRng;
use crate::MPCfg;

pub struct MPBuilder;

#[derive(Debug)]
pub enum MBErr {
    MissingScheme,
    InvalidScheme(String),
    UnknownScheme(String),
    UnknownVariant { family: String, variant: String },
    BadCfg(String), // algorithm-specific parse/validation error
}

impl std::fmt::Display for MBErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MBErr::MissingScheme => write!(f, "missing scheme"),
            MBErr::InvalidScheme(s) => write!(f, "invalid scheme '{s}'"),
            MBErr::UnknownScheme(s) => write!(f, "unknown scheme '{s}'"),
            MBErr::UnknownVariant { family, variant } => {
                write!(f, "unknown variant '{variant}' for family '{family}'")
            }
            MBErr::BadCfg(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for MBErr {}

impl MPBuilder {
    pub fn build(scheme: &str, cfg: Option<MPCfg>) -> Result<Box<dyn MPRng>, MBErr> {
        let (family, variant) = parse_scheme(scheme)?;
        let cfg = cfg.unwrap_or_default();

        match family.to_ascii_lowercase().as_str() {
            "lcg64" => {
                let rng = match variant {
                    Some(v) => Lcg64::from_variant(Some(v), Some(&cfg))?,
                    None => Lcg64::new(Some(&cfg))?,
                };
                Ok(Box::new(rng))
            }
            _ => Err(MBErr::UnknownScheme(scheme.trim().to_string())),
        }
    }
}

fn parse_scheme(scheme: &str) -> Result<(&str, Option<&str>), MBErr> {
    let s = scheme.trim();
    if s.is_empty() {
        return Err(MBErr::MissingScheme);
    }

    let mut parts = s.split("::").map(str::trim);
    let family = parts.next().unwrap_or_default();
    let variant = parts.next();
    let extra = parts.next();

    if family.is_empty() || extra.is_some() {
        return Err(MBErr::InvalidScheme(s.to_string()));
    }

    if let Some(v) = variant {
        if v.is_empty() {
            return Err(MBErr::InvalidScheme(s.to_string()));
        }
    }

    Ok((family, variant))
}
