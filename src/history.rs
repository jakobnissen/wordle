use crate::{
    response::{Placement, Response},
    LetterMask, Word,
};

// TODO: Major logic problem: A letter existing in the answer
// can be marked as Wrong, if another identical letter in the guess
// is marked Correct or Misplaced.
// Hence, a letter marked as wrong doesn't mean we can remove all
// words containing that letter.

pub struct History {
    // 5 groups of 5 bytes with the 1-based letter index. Zero if not seen yet
    matches: u32,
    // If e.g. positions 2 and 4 have known Match, then word.0 & match_mask
    // zeros out the encodings of word at positions 1, 3 and 5.
    match_mask: u32,
    wrongs: LetterMask,
    responses: Vec<Response>,
}

impl History {
    pub fn new() -> Self {
        Self {
            matches: 0,
            match_mask: 0,
            wrongs: LetterMask::new(),
            responses: vec![],
        }
    }

    pub fn add_compatible(&mut self, response: Response) {
        for (i, (letter, placement)) in response.pairs().enumerate() {
            match placement {
                Placement::Correct => {
                    let shift = 5 * i as u32;
                    let encoding = letter as u32;

                    if cfg!(debug_assertions) {
                        let old_encoding = (self.matches >> shift) & 0x1f;
                        if old_encoding != 0 && old_encoding != encoding {
                            panic!();
                        }
                    }
                    self.matches |= encoding.wrapping_shl(shift);
                    self.match_mask |= 0x1fu32.wrapping_shl(shift)
                }
                Placement::Wrong | Placement::Misplaced => {}
            }
        }
        self.wrongs = self.wrongs.union(response.get_wrong_mask());
        self.responses.push(response)
    }

    pub fn is_compatible(&self, word: Word) -> bool {
        let mask: LetterMask = word.into();
        if mask.intersects(self.wrongs) {
            return false;
        };
        if word.0 & self.match_mask != self.matches {
            return false;
        }
        // TODO: This might be a bottleneck - we should add some more
        // short paths here.
        for response in self.responses.iter() {
            if !response.matches(word) {
                return false;
            }
        }
        true
    }
}
