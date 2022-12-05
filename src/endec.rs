use crate::{word::words_iterator::WordsIterator, BlocksIterator, Key, Rc5, Rc5Error, Word};

///
/// A generic implementation of the rc5 cipher that can be configured with different Word types.
///
pub(crate) struct Endec<W: Word> {
    s: Vec<W>,
    rounds: usize,
}

impl<W> Rc5 for Endec<W>
where
    W: Word,
{
    /// Returns a cipher text for the given plain text.
    fn encode(&self, plaintext: &[u8]) -> crate::Result<Vec<u8>> {
        let mut result = Vec::with_capacity(plaintext.len());

        let bits = W::BITS.into();

        for (mut a, mut b) in BlocksIterator::<W>::try_from(plaintext)? {
            a = a.wrapping_add(&self.s[0]);
            b = b.wrapping_add(&self.s[1]);

            for i in 1..=self.rounds {
                a = (a ^ b)
                    .rotate_left((b % bits).to_u32().ok_or_else(|| {
                        Rc5Error::InternalError(String::from(
                            "error during rounding word A when encoding",
                        ))
                    })?)
                    .wrapping_add(&self.s[2 * i]);
                b = (b ^ a)
                    .rotate_left((a % bits).to_u32().ok_or_else(|| {
                        Rc5Error::InternalError(String::from(
                            "error during rounding word B when encoding",
                        ))
                    })?)
                    .wrapping_add(&self.s[2 * i + 1]);
            }

            result.extend(W::to_le_bytes(a).as_ref());
            result.extend(W::to_le_bytes(b).as_ref());
        }

        Ok(result)
    }

    /// Returns a plain text for the given cipher text.
    fn decode(&self, ciphertext: &[u8]) -> crate::Result<Vec<u8>> {
        let mut result = Vec::with_capacity(ciphertext.len());

        let bits = W::BITS.into();

        for (mut a, mut b) in BlocksIterator::<W>::try_from(ciphertext)? {
            for i in (1..=self.rounds).rev() {
                b = b.wrapping_sub(&self.s[2 * i + 1]).rotate_right(
                    (a % bits).to_u32().ok_or_else(|| {
                        Rc5Error::InternalError(String::from(
                            "error during rounding word B when dencoding",
                        ))
                    })?,
                ) ^ a;
                a = a
                    .wrapping_sub(&self.s[2 * i])
                    .rotate_right((b % bits).to_u32().ok_or_else(|| {
                        Rc5Error::InternalError(String::from(
                            "error during rounding word A when dencoding",
                        ))
                    })?)
                    ^ b;
            }

            a = a.wrapping_sub(&self.s[0]);
            b = b.wrapping_sub(&self.s[1]);

            result.extend(W::to_le_bytes(a).as_ref());
            result.extend(W::to_le_bytes(b).as_ref());
        }

        Ok(result)
    }
}

impl<W> Endec<W>
where
    W: Word,
{
    /// Creates a new Endec instance.
    pub(crate) fn setup(key: Key, rounds: usize) -> crate::Result<Self> {
        Ok(Self {
            s: Self::expand_key(key, rounds)?,
            rounds,
        })
    }

    /// Returns an expanded key table needed for further encryption/decryption.
    fn expand_key(key: Key, rounds: usize) -> crate::Result<Vec<W>> {
        let mut s: Vec<W> = vec![W::zero(); 2 * (rounds + 1)];
        let mut l: Vec<W> = WordsIterator::<W>::try_from(key.raw())?.collect();

        s[0] = W::P;
        for i in 1..s.len() {
            s[i] = s[i - 1].wrapping_add(&W::Q);
        }

        let (mut a, mut b, mut i, mut j) = (W::zero(), W::zero(), 0, 0);

        for _ in 0..3 * std::cmp::max(s.len(), l.len()) {
            a = a.wrapping_add(&b).wrapping_add(&s[i]).rotate_left(3);
            s[i] = a;

            let ab = a.wrapping_add(&b);
            b = ab
                .wrapping_add(&l[j])
                .rotate_left((ab % W::BITS.into()).to_u32().unwrap());
            l[j] = b;

            i = (i + 1) % s.len();
            j = (j + 1) % l.len();
        }

        Ok(s)
    }
}
