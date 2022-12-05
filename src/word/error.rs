use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum WordError {
    #[error("bytes input has length {0} and cannot be split into blocks of {1}")]
    InputCanNotBeSplittedByBlocks(usize, usize),
    #[error("bytes input has length {0} and cannot be split into words of {1}")]
    InputCanNotBeSplittedByWords(usize, usize),
}
