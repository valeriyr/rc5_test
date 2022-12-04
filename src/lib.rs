#![deny(unsafe_code)]

mod endec;
mod word;

pub mod error;
pub mod key;

#[cfg(test)]
mod tests;

use endec::Endec;
use word::{BlocksIterator, Word};

pub use self::error::Rc5Error;
pub use self::key::Key;

pub type Result<T> = std::result::Result<T, Rc5Error>;

pub trait Rc5 {
    fn encode(&self, plaintext: &[u8]) -> crate::Result<Vec<u8>>;
    fn decode(&self, ciphertext: &[u8]) -> crate::Result<Vec<u8>>;
}

pub fn rc5_w8(key: Key, rounds: usize) -> crate::Result<impl Rc5> {
    Endec::<word::W8>::setup(key, rounds)
}

pub fn rc5_w16(key: Key, rounds: usize) -> crate::Result<impl Rc5> {
    Endec::<word::W16>::setup(key, rounds)
}

pub fn rc5_w32(key: Key, rounds: usize) -> crate::Result<impl Rc5> {
    Endec::<word::W32>::setup(key, rounds)
}

pub fn rc5_w64(key: Key, rounds: usize) -> crate::Result<impl Rc5> {
    Endec::<word::W64>::setup(key, rounds)
}
