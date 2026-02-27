use std::io;
use thiserror::Error;

pub type MRndResult<T> = Result<T, MRndErr>;

#[derive(Error, Debug)]
pub enum MRndErr {
    #[error("config: missing key '{0}'")]
    MissingCfg(&'static str),
    #[error("config error: '{0}'")]
    BadCfg(String),
    #[error("parse error: '{0}'")]
    ParseErr(String),

    #[error("config: invalid schema '{0}'")]
    InvalidSchema(String),

    //#[error("config: Unkonwn BigGenerator Engine '{0}'")]
    #[error("confg: The BigGenerator Engine {wrong_engine} is unknown and not supported. Available engines: {available:?}")]
    UnknownEngine{
        wrong_engine: String,
        available: Vec<&'static str>,
    },

    #[error("engine: unknown preset '{preset}' for '{engine}'. Available: {available:?}")]
    UnknownPreset {
        engine: String,
        preset: String,
        available: Vec<&'static str>,
    },

    #[error("failed to open entropy source {path}: {source}")]
    EntropyOpen {
        path: &'static str,
        #[source]
        source: io::Error,
    },

    #[error("entropy error: {0}")]
    Entropy(#[from] io::Error),

    #[error("internal error: {0}")]
    Internal(String),
}
