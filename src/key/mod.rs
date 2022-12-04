pub mod error;

pub use self::error::KeyError;

#[derive(Debug)]
pub struct Key {
    value: Vec<u8>,
}

impl Key {
    pub fn raw(&self) -> &[u8] {
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

        assert!(matches!(
            Key::try_from(key.as_ref()),
            Err(KeyError::InvalidLength(_))
        ));
    }

    #[test]
    fn too_long_key() {
        let key = vec![0; 257];

        assert!(matches!(
            Key::try_from(key.as_ref()),
            Err(KeyError::InvalidLength(_))
        ));
    }

    #[test]
    fn correct_key() {
        let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];

        assert!(Key::try_from(key.as_ref()).is_ok());
    }
}
