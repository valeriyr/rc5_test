use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum KeyError {
    #[error("invalid key length {0}, expected in the range from 1 to 256")]
    InvalidLength(usize),
}
