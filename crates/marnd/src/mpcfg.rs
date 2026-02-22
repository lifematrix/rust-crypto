use std::collections::HashMap;
use crate::MBErr;
use std::simd::f32x4;


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
        self.c.get(k).map(|s| s.as_str())
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.c.keys().map(|s| s.as_str())
    }

    pub fn get_u64(&self, key: &str) -> Result<Option<u64>, MBErr> {
        let v = match self.get(key) {
            Some(v) => v,
            None => return Ok(None),
        };

        let s = v.trim();
        if s.is_empty() {
            return Err(crate::MBErr::BadCfg(format!(
                "lcg64: '{key}' cannot be empty"
            )));
        }

        let val = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            u64::from_str_radix(hex, 16)
        } else {
            s.parse::<u64>()
        };

        val.map(Some).map_err(|_| {
            MBErr::BadCfg(format!(
                "lcg64: invalid u64 for '{key}': '{s}'"
            ))
        })
    }

    pub fn get_u64_or(
        &self,
        key: &str,
        default: u64,
    ) -> Result<u64, MBErr> {
        Ok(self.get_u64(key)?.unwrap_or(default))
    }
}
