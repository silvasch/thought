use std::str::FromStr;

use rand::{seq::SliceRandom, thread_rng};

use crate::{Error, WORDS};

#[derive(Clone, Debug, PartialEq)]
pub struct ThoughtId {
    first_word: String,
    second_word: String,
}

impl ThoughtId {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let first_word = WORDS.choose(&mut rng).expect("WORDS is not empty.");
        let second_word = WORDS.choose(&mut rng).expect("WORDS is not empty.");
        Self {
            first_word: first_word.to_string(),
            second_word: second_word.to_string(),
        }
    }
}

impl FromStr for ThoughtId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split('-').collect();

        if split.len() != 2 {
            return Err(Error::ParseId {
                invalid_thought_id: s.to_string(),
            });
        }

        let first_word = split[0];
        let second_word = split[1];

        Ok(ThoughtId {
            first_word: first_word.to_string(),
            second_word: second_word.to_string(),
        })
    }
}

impl std::fmt::Display for ThoughtId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.first_word, self.second_word)
    }
}
