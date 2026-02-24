use std::collections::HashMap;
use crate::MRndErr;
use std::num::ParseIntError;

pub struct CfgUtil;

impl CfgUtil {
    /// Encapsulates u64 parsing logic (Decimal and Hex).
    pub fn parse_u64(s: &str) -> Result<u64, ParseIntError> {
        if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            u64::from_str_radix(hex, 16)
        } else {
            s.parse::<u64>()
        }
    }

    pub fn parse_schema(schema: &str) -> Result<(&str, Option<&str>), MRndErr> {
        let (engine, preset) = match schema.split_once("::") {
            Some((e,p)) => (e.trim(), Some(p.trim())),
            None => (schema, None)
        };

        Ok((engine, preset))
    }
    
}

#[derive(Clone, Debug, Default)]
pub struct MPCfg {
    c: HashMap<String, String>,
}

impl MPCfg {
    pub fn new() -> Self {
        Self { c: HashMap::new() }
    }

    pub fn insert(&mut self, k: &str, v: &str) -> Option<String> {
        // to_owned() converts &str into an owned String for storage.
        self.c.insert(k.to_owned(), v.to_owned())
    }

    pub fn get(&self, k: &str) -> Option<&str> {
        self.c.get(k).map(|s| s.as_str().trim())
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.c.keys().map(|s| s.as_str())
    }

    pub fn get_str(&self, key: &str, required: bool) -> Result<Option<&str>, MRndErr> {
        match self.get(key) {
            None =>  {
                if required {
                    Err(MRndErr::BadCfg(format!("Missing key '{}'", key)))
                }
                else {
                    Ok(None)
                }
            }
            Some(s) => {
                if s.is_empty() {
                    Err(MRndErr::BadCfg(format!("The value of key '{}' is empty", key)))
                }
                else {
                    Ok(Some(s))
                }
            }
        }
    }

    pub fn get_u64(&self, key: &str, required: bool) -> Result<Option<u64>, MRndErr> {
        let s = match self.get_str(key, required)? {
            Some(val) => val,
            None => return Ok(None)
        };

        CfgUtil::parse_u64(s).map(Some).map_err(|_| {
            MRndErr::ParseErr(format!("Invalid u64 for '{}': '{}'", key, s))
        })
    }

    // pub fn get_u64_or(
    //     &self,
    //     key: &str,
    //     default: u64,
    // ) -> Result<u64, MRndErr> {
    //     Ok(self.get_u64(key, false)?.unwrap_or(default))
    // }
}
