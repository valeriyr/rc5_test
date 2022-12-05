pub mod blocks_iterator;
pub mod error;
pub mod words_iterator;

use num_traits::{PrimInt, WrappingAdd, WrappingShl, WrappingShr, WrappingSub};

pub use self::error::WordError;

///
/// The trait can be implemented for words of different lengths.
///
pub trait Word:
    WrappingShr + WrappingShl + WrappingAdd + WrappingSub + PrimInt + From<u8> + std::fmt::Debug
{
    const BITS: u8;
    const BYTES: u8 = Self::BITS / 8;

    const P: Self;
    const Q: Self;

    type Bytes: TryFrom<Vec<u8>> + AsRef<[u8]>;

    fn to_le_bytes(w: Self) -> Self::Bytes;
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
}

///
/// Generates a 'Word' implementation with the specified parameters.
///
macro_rules! decl_word {
    ($name:ident, $type:ty, $p:literal, $q:literal) => {
        pub type $name = $type;

        impl Word for $type {
            const BITS: u8 = Self::BITS as u8;

            const P: Self = $p;
            const Q: Self = $q;

            type Bytes = [u8; Self::BYTES as usize];

            fn to_le_bytes(w: Self) -> Self::Bytes {
                Self::to_le_bytes(w)
            }

            fn from_le_bytes(bytes: Self::Bytes) -> Self {
                Self::from_le_bytes(bytes)
            }
        }
    };
}

decl_word!(W8, u8, 0xB7, 0x9F);
decl_word!(W16, u16, 0xB7E1, 0x9E37);
decl_word!(W32, u32, 0xB7E15163, 0x9E3779B9);
decl_word!(W64, u64, 0xB7E151628AED2A6B, 0x9E3779B97F4A7C15);
