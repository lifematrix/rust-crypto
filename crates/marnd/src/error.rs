use thiserror::Error;
use std::io;

pub type MRndResult<T> = Result<T, MRndErr>;

#[derive(Error, Debug)]
pub enum MRndErr {
    #[error("config: missing key '{0}'")]
    MissingCfg(&'static str),
    #[error("config error: '{0}'")]
    BadCfg(String),
    // #[error("config: invalid value for '{key}': {msg}")]
    // BadCfg {
    //     key: String,
    //     msg: String
    // },
    #[error("parse error: '{0}'")]
    ParseErr(String),

    #[error("config: invalid schema '{0}'")]
    InvalidSchema(String),

    #[error("config: Unkonwn BigGenerator Engine '{0}'")]
    UnknownEngine(String),

    // #[error("config: Unkonwn Preset of Engine '{0}'")]
    // UnknownPreset(String),

    #[error("engine: unknown preset '{preset}' for '{engine}'. Available: {available:?}")]
    UnknownPreset {
        engine: String,
        preset: String, 
        available: Vec<&'static str>,
    },

    #[error("entropy error: {0}")]
    Entropy(#[from] io::Error),

    #[error("internal error: {0}")]
    Internal(String),
}