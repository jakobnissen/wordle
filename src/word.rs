use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
// In groups of 5 bytes, from lowest to highest,
// where A is 1 and Z is 26
pub struct Word(pub u32);

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8(self.into_iter().map(|x| x + b'A' - 1).collect()).unwrap()
        )
    }
}

pub struct WordIterator(u32);

impl Iterator for WordIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let u = (self.0 & 0x0000001f) as u8;
            self.0 = self.0.wrapping_shr(5);
            Some(u)
        }
    }
}

impl<'a> IntoIterator for &'a Word {
    type Item = u8;
    type IntoIter = WordIterator;

    fn into_iter(self) -> Self::IntoIter {
        WordIterator(self.0)
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
        if bytes.len() != 5 {
            return Err(ParseWordError::WrongSize(bytes.len()));
        }
        let mut u: u32 = 0;
        for (i, b) in bytes.iter().enumerate().rev() {
            let encoding = b.wrapping_sub(b'A' + 32 * b.is_ascii_lowercase() as u8);
            if encoding > 25 {
                return Err(ParseWordError::NotLetter(i));
            }
            u = (u << 5) | (encoding as u32 + 1);
        }
        Ok(Word(u))
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
            assert_eq!(w.to_string(), word.to_uppercase())
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
