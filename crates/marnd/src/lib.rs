pub mod error;
pub mod mpcfg;
pub mod mprng;


pub mod entropy {
    pub mod mosentropy;
}

pub(crate) mod bitgen {
    pub(crate) mod mbitgen;
    pub(crate) mod lcg64;
}

pub use crate::bitgen::lcg64::Lcg64;
pub use crate::error::MRndErr;
pub use crate::entropy::mosentropy::MOSEntropy;
pub use crate::mpcfg::CfgUtil;
pub use crate::mpcfg::MPCfg;
pub use crate::bitgen::mbitgen::MBitGen;
