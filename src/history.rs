use crate::{
    response::{Placement, Response},
    LetterMask, Word,
};

pub struct History {
    matches: [Option<u8>; 5],
    wrongs: LetterMask,
    responses: Vec<Response>,
}

impl History {
    pub fn new() -> Self {
        Self {
            matches: [None; 5],
            wrongs: LetterMask::new(),
            responses: vec![],
        }
    }

    pub fn add_compatible(&mut self, response: Response) {
        for ((letter, placement), matc) in response.pairs().zip(self.matches.iter_mut()) {
            match placement {
                Placement::Correct => {
                    if cfg!(debug_assertions) && matc.is_some_and(|x| x != letter) {
                        panic!("{:?}", matc);
                    }
                    *matc = Some(letter)
                }
                Placement::Wrong => self.wrongs = self.wrongs.add(letter),
                Placement::Misplaced => {}
            }
        }
        self.responses.push(response)
    }

    pub fn is_compatible(&self, word: Word) -> bool {
        let mask: LetterMask = word.into();
        if mask.intersects(self.wrongs) {
            return false;
        };
        for (maybe_match, letter) in self.matches.iter().zip(word.into_iter()) {
            if maybe_match.is_some_and(|m| m != letter) {
                return false;
            }
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
