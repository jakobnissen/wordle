use crate::LetterMask;

use super::Word;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Placement {
    Correct,
    Misplaced,
    Wrong,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Response {
    pub guess: Word,
    placements: [Placement; 5],
}

impl Response {
    pub fn new(guess: Word, answer: Word) -> Self {
        let mask: LetterMask = answer.into();
        // Positions in the guess that is not used. This matters since a misplaced letter
        // can be signalled as misplaced only once, and not if it's correct or wrong.
        let mut unused_answer: u32 = 0b11111;
        let mut potential_misplaced = 5;
        let mut placements: [Option<Placement>; 5] = [None; 5];
        for (i, ((g, a), p)) in guess
            .into_iter()
            .zip(answer.into_iter())
            .zip(placements.iter_mut())
            .enumerate()
        {
            // If the letter is correct, and correctly placed: Set it as correct
            if g == a {
                *p = Some(Placement::Correct);
                unused_answer &= !1u32.wrapping_shl(i as u32);
                potential_misplaced -= 1;
            // If this letter in the guess is not present in the answer, it's wrong
            } else if !mask.contains(g) {
                *p = Some(Placement::Wrong);
                potential_misplaced -= 1;
            }
        }
        // If all letters are correct or wrong, we don't need to check for misplaced.
        if potential_misplaced != 0 {
            for (g, p) in guess.into_iter().zip(placements.iter_mut()) {
                // If it's none, it's not Correct or Wrong
                if p.is_none() {
                    // By default, we assume that this letter has already been marked as
                    // Misplaced in some previous iteration.
                    *p = Some(Placement::Wrong);
                    for (a_i, a) in answer.into_iter().enumerate() {
                        if g == a && unused_answer.wrapping_shr(a_i as u32) & 1 == 1 {
                            unused_answer &= !1u32.wrapping_shl(a_i as u32);
                            *p = Some(Placement::Misplaced);
                            break;
                        }
                    }
                }
            }
        }
        Self {
            guess,
            placements: placements.map(|x| x.unwrap()),
        }
    }

    pub fn matches(&self, answer: Word) -> bool {
        Self::new(self.guess, answer) == *self
    }

    pub fn pairs(&self) -> impl Iterator<Item = (u8, Placement)> {
        self.guess.into_iter().zip(self.placements)
    }

    /// We take the ones marked as wrong, and subtract the letters marked as correct
    /// or misplaced. This is because correct/misplaced letters may also be marked as
    /// wrong if there are multiple copies of them in the guess.
    pub fn get_wrong_mask(&self) -> LetterMask {
        let mut wrong = LetterMask::new();
        let mut c_or_m = LetterMask::new();
        for (letter, placement) in self.guess.into_iter().zip(self.placements) {
            match placement {
                Placement::Wrong => wrong = wrong.add(letter),
                Placement::Correct | Placement::Misplaced => c_or_m = c_or_m.add(letter),
            }
        }
        wrong.diff(c_or_m)
    }

    #[allow(dead_code)]
    pub fn to_debug_string(&self) -> String {
        self.placements
            .iter()
            .map(|p| match p {
                Placement::Correct => 'C',
                Placement::Wrong => 'W',
                Placement::Misplaced => 'M',
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response() {
        for (guess, answer, response) in [
            ("AAAAA", "AAAAA", "CCCCC"),
            ("AAAAA", "BBBBB", "WWWWW"),
            ("AABAB", "AACAC", "CCWCW"),
            ("ABCDE", "BCDEA", "MMMMM"),
            ("ABCDE", "AXEDC", "CWMCM"),
            ("ABCDA", "BAXXA", "MMWWC"),
            ("ABABA", "BAABA", "MMCCC"),
            ("ABCAC", "BBCAC", "WCCCC"),
            ("ABCDA", "ABCDE", "CCCCW"),
            ("ZYMIC", "WIMPY", "WMCMW"),
            ("XOANA", "ORGAN", "WMMMW"),
        ] {
            let r = Response::new(
                guess.as_bytes().try_into().unwrap(),
                answer.as_bytes().try_into().unwrap(),
            );
            assert_eq!(r.to_debug_string(), response)
        }
    }
}
