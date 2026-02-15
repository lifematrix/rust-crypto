pub mod mpbuilder;
pub mod mpcfg;
pub mod mprng;

pub use crate::mpbuilder::MBErr;
pub use crate::mpbuilder::MPBuilder;
pub use crate::mpcfg::MPCfg;
pub use crate::mprng::MPRng;
pub use crate::mprng::MPRngExt;

pub(crate) mod lcg {
    pub(crate) mod lcg64;
}

pub mod entropy {
    pub mod mosentropy;
}

pub use crate::entropy::mosentropy::MOSEntropy;
