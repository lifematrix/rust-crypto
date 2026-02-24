use marcore::OptionExt;
use crate::{CfgUtil, MBitGen, MOSEntropy, MPCfg, MRndErr, Lcg64};


#[derive(Debug)]
pub struct MPRng {
    // bitgen: Box<dyn MBitGen>,
    bitgen: Box<dyn MBitGen>,
}

impl MPRng {
    pub fn new(bitgen: Box<dyn MBitGen>) -> Self {
        Self { bitgen }
    }

    pub fn build(cfg: &MPCfg) -> Result<Self, MRndErr> {
        let schema = cfg.get_str("schema", true)?.unwrap();
        let (engine, preset) = CfgUtil::parse_schema(schema)?;

        let seed = cfg.get_u64("seed", false)?.or_try(|| MOSEntropy::next_u64())?;
        // let seed = match cfg.get_u64("seed", false)? {
        //     Some(s) => s,
        //     None => MOSEntropy::next_u64()?
        // };

        let bitgen = match engine {
            "Lcg64" => match preset {
                Some(p) => Lcg64::from_preset(p, seed)
                                    .ok_or_else(|| MRndErr::UnknownPreset 
                                        { engine: engine.into(), 
                                        preset: p.into(), 
                                        available: Lcg64::list_presets()}),
                None => {
                    let a = cfg.get_u64("a", true)?.unwrap();
                    let c = cfg.get_u64("c", true)?.unwrap();
                    Ok(Lcg64::new(a, c, seed))
                }
            }
            _ => Err(MRndErr::UnknownEngine(format!("The engine {} is unknown and not supported", engine)))
        }?;

        Ok(Self::new(Box::new(bitgen)))
    }
}

