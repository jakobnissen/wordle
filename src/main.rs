mod history;
mod naive;
mod response;
mod word;

use history::History;
use response::Response;
use word::Word;

use std::collections::HashSet;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time;

#[derive(Clone, Copy)]
struct LetterMask(u32);

impl LetterMask {
    fn new() -> Self {
        Self(0u32)
    }

    fn add(self, letter: u8) -> Self {
        Self(self.0 | 1u32.wrapping_shl(letter as u32))
    }

    fn contains(self, letter: u8) -> bool {
        self.0.wrapping_shr(letter as u32) & 1 == 1
    }

    fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    fn diff(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl From<Word> for LetterMask {
    fn from(value: Word) -> Self {
        let mut mask = Self::new();
        for i in value.into_iter() {
            mask = mask.add(i)
        }
        mask
    }
}

trait Solver {
    fn new(valid_words: &[Word]) -> Self;
    fn reset(&mut self);
    fn guess(&mut self, history: &History) -> Word;
}

fn play<G: Solver>(solver: &mut G, answer: Word) -> Option<usize> {
    solver.reset();
    // dbg!(answer.to_string());
    let mut history = History::new();
    for attempt in 1..=64 {
        let guess = solver.guess(&history);
        // dbg!(guess.to_string());
        if guess == answer {
            return Some(attempt);
        } else {
            let response = Response::new(guess, answer);
            // dbg!(response.clone().to_debug_string());
            history.add_compatible(response)
        }
    }
    None
}

fn main() {
    let valid_words: Vec<Word> = include_str!("../words/valid.txt")
        .lines()
        .map(|line| line.as_bytes().try_into().unwrap())
        .collect();
    let answer_words: Vec<Word> = include_str!("../words/answers.txt")
        .lines()
        .map(|line| line.as_bytes().try_into().unwrap())
        .collect();
    if cfg!(debug_assertions) {
        assert!(answer_words
            .iter()
            .collect::<HashSet<_>>()
            .is_subset(&valid_words.iter().collect::<HashSet<_>>()))
    }
    let mut solver = naive::Naive::new(&valid_words);
    let mut iterations: Vec<Option<usize>> = Vec::new();
    let mut rng = thread_rng();
    let start = time::Instant::now();
    let n_iterations = 10000;
    for _ in 0..n_iterations {
        let answer = answer_words.choose(&mut rng).unwrap();
        iterations.push(play(&mut solver, *answer));
        solver.reset();
    }
    let duration = time::Instant::now().duration_since(start);
    println!(
        "Solved {} iteration in {} ms",
        n_iterations,
        duration.as_millis()
    );
    println!(
        "   Number of fails: {}",
        iterations.iter().filter(|x| x.is_none()).count()
    );
    let passed: Vec<_> = iterations.iter().flatten().copied().collect();
    if !passed.is_empty() {
        println!("   Min: {}", passed.iter().min().unwrap());
        println!("   Max: {}", passed.iter().max().unwrap());
        println!(
            "   Mean: {}",
            passed.iter().sum::<usize>() as f64 / passed.len() as f64
        );
    }
}
