use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Word {
    bytes: [u8; 5],
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.bytes.as_slice()).unwrap())
    }
}

impl<'a> IntoIterator for &'a Word {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, 5>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter()
    }
}

#[derive(Debug)]
pub enum ParseWordError {
    WrongSize(usize),
    NotLetter(usize),
}

impl TryFrom<&[u8]> for Word {
    type Error = ParseWordError;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut arr: [u8; 5] = match bytes.try_into() {
            Ok(a) => a,
            Err(_) => return Err(ParseWordError::WrongSize(bytes.len())),
        };
        for (i, b) in arr.iter_mut().enumerate() {
            if b.is_ascii_lowercase() {
                *b -= 32u8;
            }
            if !b.is_ascii_uppercase() {
                return Err(ParseWordError::NotLetter(i));
            }
        }
        Ok(Word { bytes: arr })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        for word in vec!["ABCDE", "AKROD", "Sofow", "pwpgm"] {
            assert!(TryInto::<Word>::try_into(word.as_bytes()).is_ok());
            let w: Word = word.as_bytes().try_into().unwrap();
            assert!(w.into_iter().all(|b| (b'A'..=b'Z').contains(&b)));
        }
        for bad_length in vec!["ABCD", "ABCDEF", "", "æææææ"] {
            assert!(matches!(
                TryInto::<Word>::try_into(bad_length.as_bytes()),
                Err(ParseWordError::WrongSize(_))
            ))
        }
        for bad_word in vec!["ABC!D", "ææ1", "k1amv"] {
            assert!(matches!(
                TryInto::<Word>::try_into(bad_word.as_bytes()),
                Err(ParseWordError::NotLetter(_))
            ))
        }
    }
}
