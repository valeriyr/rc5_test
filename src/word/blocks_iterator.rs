use std::marker::PhantomData;

use super::{Word, WordError};

pub struct BlocksIterator<'a, W: Word> {
    bytes: &'a [u8],
    index: usize,
    _w: PhantomData<W>,
}

impl<W> Iterator for BlocksIterator<'_, W>
where
    W: Word,
{
    type Item = (W, W);

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.len() > self.index {
            let bytes_in_word: usize = W::BYTES.into();

            // TODO: this check should be always passed because the bytes size is checked.
            //       maybe it is better to look for some crates instead using the 'from_le_bytes'
            //       function that works with statically sized arrays.
            if let Ok(bytes1) = self.bytes[self.index..self.index + bytes_in_word]
                .to_owned()
                .try_into()
            {
                if let Ok(bytes2) = self.bytes
                    [self.index + bytes_in_word..self.index + 2 * bytes_in_word]
                    .to_owned()
                    .try_into()
                {
                    let word1 = W::from_le_bytes(bytes1);
                    let word2 = W::from_le_bytes(bytes2);

                    self.index += 2 * bytes_in_word;

                    return Some((word1, word2));
                }
            }

            None
        } else {
            None
        }
    }
}

impl<'a, W> TryFrom<&'a [u8]> for BlocksIterator<'a, W>
where
    W: Word,
{
    type Error = WordError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let input_len = bytes.len();
        let block_len: usize = (W::BYTES * 2).into();

        if input_len % block_len > 0 {
            Err(WordError::InputCanNotBeSplittedByBlocks(
                input_len, block_len,
            ))
        } else {
            Ok(Self {
                bytes,
                index: 0,
                _w: PhantomData,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_bytes_input() {
        let bytes = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66];

        assert!(matches!(
            BlocksIterator::<crate::word::W32>::try_from(bytes.as_ref()),
            Err(WordError::InputCanNotBeSplittedByBlocks(7, 8))
        ));
    }

    #[test]
    fn correct_bytes_input() {
        let bytes = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];

        let mut iterator = BlocksIterator::<crate::word::W32>::try_from(bytes.as_ref()).unwrap();

        assert_eq!(iterator.next(), Some((857870592, 2003195204)));
        assert_eq!(iterator.next(), None);
    }
}
