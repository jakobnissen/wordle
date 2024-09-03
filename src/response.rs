use crate::LetterMask;

use super::Word;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    Correct,
    Misplaced,
    Wrong,
}

#[derive(PartialEq, Eq)]
pub struct Response {
    pub guess: Word,
    placements: [Placement; 5],
}

impl Response {
    pub fn new(guess: Word, answer: Word) -> Self {
        let mask: LetterMask = answer.into();
        // Positions in the guess that is not used. This matters since a misplaced letter
        // can be signalled as misplaced only once, and not if it's correct or wrong.
        let mut unused: u8 = 0b11111;
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
                unused &= !1u8.wrapping_shl(i as u32);
            // If this letter in the guess is not present in the answer, it's wrong
            } else if !mask.contains(g) {
                *p = Some(Placement::Wrong);
                unused &= !1u8.wrapping_shl(i as u32);
            }
        }
        // If all letters are correct or wrong, we don't need to check for misplaced.
        if unused != 0 {
            for (guess_index, (g, p)) in guess.into_iter().zip(placements.iter_mut()).enumerate() {
                // If it's none, it's not Correct or Wrong
                if p.is_none() {
                    // TODO: Could short-circuit here if used is zero
                    for a in answer.into_iter() {
                        if g == a && unused.wrapping_shr(guess_index as u32) & 1 == 1 {
                            unused &= !1u8.wrapping_shl(guess_index as u32);
                            *p = Some(Placement::Misplaced);
                            break;
                        }
                    }
                    if p.is_none() {
                        *p = Some(Placement::Wrong)
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
}
