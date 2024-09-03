use crate::history::History;

use super::{Solver, Word};

pub struct Naive {
    words: Vec<Word>,
    remaining: Vec<Word>,
}

impl Solver for Naive {
    fn new(valid_words: &[Word]) -> Self {
        Self {
            words: valid_words.to_vec(),
            remaining: valid_words.to_vec(),
        }
    }

    fn reset(&mut self) {
        self.remaining.clear();
        self.remaining.extend_from_slice(&self.words)
    }

    fn guess(&mut self, history: &History) -> Word {
        while let Some(candidate) = self.remaining.pop() {
            if history.is_compatible(candidate) {
                return candidate;
            } else {
                //dbg!(candidate.to_string());
            }
        }
        panic!("No more available candidates");
    }
}
