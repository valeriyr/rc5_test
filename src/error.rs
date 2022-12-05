use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Rc5Error {
    #[error("internal error: {0}")]
    InternalError(String),
    #[error("key error: {0}")]
    KeyError(#[from] crate::key::KeyError),
    #[error("word error: {0}")]
    WordError(#[from] crate::word::WordError),
}
