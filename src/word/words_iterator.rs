use std::marker::PhantomData;

use super::{Word, WordError};

/// The iterator can be used to iterate over an array of bytes word by word.
pub struct WordsIterator<'a, W: Word> {
    bytes: &'a [u8],
    index: usize,
    _w: PhantomData<W>,
}

impl<W> Iterator for WordsIterator<'_, W>
where
    W: Word,
{
    type Item = W;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.len() > self.index {
            let bytes_in_word: usize = W::BYTES.into();

            // TODO: this check should be always passed because the bytes size is checked.
            //       maybe it is better to look for some crates instead using the 'from_le_bytes'
            //       function that works with statically sized arrays.
            if let Ok(bytes) = self.bytes[self.index..self.index + bytes_in_word]
                .to_owned()
                .try_into()
            {
                let word = W::from_le_bytes(bytes);

                self.index += bytes_in_word;

                return Some(word);
            }

            None
        } else {
            None
        }
    }
}

impl<'a, W> TryFrom<&'a [u8]> for WordsIterator<'a, W>
where
    W: Word,
{
    type Error = WordError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let input_len = bytes.len();
        let word_len: usize = W::BYTES.into();

        if input_len % word_len > 0 {
            Err(WordError::InputCanNotBeSplittedByWords(input_len, word_len))
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

        assert_eq!(
            WordsIterator::<crate::word::W32>::try_from(bytes.as_ref())
                .err()
                .unwrap(),
            WordError::InputCanNotBeSplittedByWords(7, 4)
        );
    }

    #[test]
    fn correct_bytes_input() {
        let bytes = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];

        let mut iterator = WordsIterator::<crate::word::W32>::try_from(bytes.as_ref()).unwrap();

        assert_eq!(iterator.next(), Some(857870592));
        assert_eq!(iterator.next(), Some(2003195204));
        assert_eq!(iterator.next(), None);
    }
}
