pub(crate) mod error;
pub(crate) mod mpcfg;
pub(crate) mod mprng;

pub(crate) mod entropy {
    pub(crate) mod mosentropy;
}

pub(crate) mod bitgen {
    pub(crate) mod lcg64;
    pub(crate) mod mbitgen;
}

pub(crate) use crate::bitgen::mbitgen::MBitGen;
pub(crate) use crate::mpcfg::CfgUtil;

pub use crate::bitgen::lcg64::Lcg64;
pub use crate::entropy::mosentropy::MOSEntropy;
pub use crate::error::MRndErr;
pub use crate::mpcfg::MPCfg;
pub use crate::mprng::MPRng;
