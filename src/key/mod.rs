mod error;

pub use self::error::KeyError;

/// Implementation of the rc5 key. Validates possible key length ranges from 1 to 256.
#[derive(Debug)]
pub struct Key {
    value: Vec<u8>,
}

impl Key {
    /// Raw data accessor.
    pub(crate) fn raw(&self) -> &[u8] {
        &self.value
    }
}

impl TryFrom<&[u8]> for Key {
    type Error = KeyError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let len = value.len();

        if len == 0 || len > 256 {
            Err(KeyError::InvalidLength(len))
        } else {
            Ok(Self {
                value: value.to_vec(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_size_key() {
        let key = vec![];

        assert_eq!(
            Key::try_from(key.as_ref()).unwrap_err(),
            KeyError::InvalidLength(0)
        );
    }

    #[test]
    fn one_byte_key() {
        let key = vec![0x00];

        assert!(Key::try_from(key.as_ref()).is_ok());
    }

    #[test]
    fn largest_key() {
        let key = vec![0; 256];

        assert!(Key::try_from(key.as_ref()).is_ok());
    }

    #[test]
    fn too_long_key() {
        let key = vec![0; 257];

        assert_eq!(
            Key::try_from(key.as_ref()).unwrap_err(),
            KeyError::InvalidLength(257)
        );
    }

    #[test]
    fn correct_key() {
        let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];

        assert!(Key::try_from(key.as_ref()).is_ok());
    }
}
