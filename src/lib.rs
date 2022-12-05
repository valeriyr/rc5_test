//! # RC5
//!
//! A Rust implementation of the rc5 cipher for word sizes of 8, 16, 32 and 64 bits.
//!
//!

#![deny(unsafe_code)]

mod endec;
mod error;
mod key;
mod word;

use endec::Endec;

pub use self::error::Rc5Error;
pub use self::key::Key;
pub use self::word::WordError;

pub type Result<T> = std::result::Result<T, Rc5Error>;

/// The rc5 cipher interface.
pub trait Rc5 {
    /// Returns a cipher text for the given plain text.
    /// Can be called multiple times for decoding text with the same key.
    fn encode(&self, plaintext: &[u8]) -> crate::Result<Vec<u8>>;

    /// Returns a plain text for the given cipher text.
    /// Can be called multiple times for decoding text with the same key.
    fn decode(&self, ciphertext: &[u8]) -> crate::Result<Vec<u8>>;
}

/// Creates an rc5 cipher instance for an 8-bit word.
///
/// Examples
///
/// ```
/// use rc5_test::{rc5_w8, Key, Rc5};
///
/// let key = vec![0x00, 0x01, 0x02, 0x03];
/// let pt = vec![
///     0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01,
/// ];
/// let ct = vec![
///     0x21, 0x2A, 0x21, 0x2A, 0x21, 0x2A, 0x21, 0x2A, 0x21, 0x2A, 0x21, 0x2A, 0x21, 0x2A,
/// ];
///
/// let rc5 = rc5_w8(Key::try_from(key.as_ref()).unwrap(), 12).unwrap();
///
/// let res = rc5.encode(&pt).unwrap();
///
/// assert_eq!(&ct[..], &res[..]);
///
/// let res = rc5.decode(&ct).unwrap();
///
/// assert_eq!(&pt[..], &res[..]);
/// ```
pub fn rc5_w8(key: Key, rounds: usize) -> crate::Result<impl Rc5> {
    Endec::<word::W8>::setup(key, rounds)
}

/// Creates an rc5 cipher instance for a 16-bit word.
///
/// Examples
///
/// ```
/// use rc5_test::{rc5_w16, Key, Rc5};
///
/// let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
/// let pt = vec![0x00, 0x01, 0x02, 0x03];
/// let ct = vec![0x23, 0xA8, 0xD7, 0x2E];
///
/// let rc5 = rc5_w16(Key::try_from(key.as_ref()).unwrap(), 16).unwrap();
///
/// let res = rc5.encode(&pt).unwrap();
///
/// assert_eq!(&ct[..], &res[..]);
///
/// let res = rc5.decode(&ct).unwrap();
///
/// assert_eq!(&pt[..], &res[..]);
/// ```
pub fn rc5_w16(key: Key, rounds: usize) -> crate::Result<impl Rc5> {
    Endec::<word::W16>::setup(key, rounds)
}

/// Creates an rc5 cipher instance for a 32-bit word.
///
/// Examples
///
/// ```
/// use rc5_test::{rc5_w32, Key, Rc5};
///
/// let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
///     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
/// let pt = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
/// let ct = vec![0x2D, 0xDC, 0x14, 0x9B, 0xCF, 0x08, 0x8B, 0x9E];
///
/// let rc5 = rc5_w32(Key::try_from(key.as_ref()).unwrap(), 12).unwrap();
///
/// let res = rc5.encode(&pt).unwrap();
///
/// assert_eq!(&ct[..], &res[..]);
///
/// let res = rc5.decode(&ct).unwrap();
///
/// assert_eq!(&pt[..], &res[..]);
/// ```
pub fn rc5_w32(key: Key, rounds: usize) -> crate::Result<impl Rc5> {
    Endec::<word::W32>::setup(key, rounds)
}

/// Creates an rc5 cipher instance for a 64-bit word.
///
/// Examples
///
/// ```
/// use rc5_test::{rc5_w64, Key, Rc5};
///
/// let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
///     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
///     0x12, 0x13, 0x14, 0x15, 0x16, 0x17];
/// let pt = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
///     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
/// let ct = vec![0xA4, 0x67, 0x72, 0x82, 0x0E, 0xDB, 0xCE, 0x02,
///     0x35, 0xAB, 0xEA, 0x32, 0xAE, 0x71, 0x78, 0xDA];
///
/// let rc5 = rc5_w64(Key::try_from(key.as_ref()).unwrap(), 24).unwrap();
///
/// let res = rc5.encode(&pt).unwrap();
///
/// assert_eq!(&ct[..], &res[..]);
///
/// let res = rc5.decode(&ct).unwrap();
///
/// assert_eq!(&pt[..], &res[..]);
/// ```
pub fn rc5_w64(key: Key, rounds: usize) -> crate::Result<impl Rc5> {
    Endec::<word::W64>::setup(key, rounds)
}
