pub mod mpcfg;
pub mod mpbuilder;
pub mod mprng;

pub use crate::mpcfg::MPCfg;
pub use crate::mpbuilder::MPBuilder;
pub use crate::mpbuilder::MBErr;
pub use crate::mprng::MPRng;

pub(crate) mod lcg {
    pub(crate) mod lcg64;
}
