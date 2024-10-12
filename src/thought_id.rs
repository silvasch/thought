use std::str::FromStr;

use chrono::{DateTime, Utc};
use rand::{seq::SliceRandom, thread_rng};

use crate::Error;

#[derive(Debug, PartialEq)]
pub struct ThoughtId {
    pub date_time: DateTime<Utc>,
    pub first_word: String,
    pub second_word: String,
}

impl ThoughtId {
    pub fn new() -> Self {
        let date_time = Utc::now();

        let mut rng = thread_rng();
        let first_word = crate::WORDS.choose(&mut rng).unwrap().to_string();
        let second_word = crate::WORDS.choose(&mut rng).unwrap().to_string();

        Self {
            date_time,
            first_word,
            second_word,
        }
    }

    pub fn get_user_id(&self) -> String {
        format!("{}-{}", self.first_word, self.second_word)
    }
}

impl std::fmt::Display for ThoughtId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            self.date_time.timestamp(),
            self.first_word,
            self.second_word
        )
    }
}

impl FromStr for ThoughtId {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = value.split('-').collect();

        if split.len() != 3 {
            return Err(Error::ParseThoughtId {
                invalid_thought_id: value.to_string(),
            });
        }

        let raw_timestamp = split.first().unwrap();
        let raw_first_word = split.get(1).unwrap();
        let raw_second_word = split.get(2).unwrap();

        let date_time = DateTime::from_timestamp(
            raw_timestamp
                .parse::<i64>()
                .map_err(|_| Error::ParseThoughtId {
                    invalid_thought_id: value.to_string(),
                })?,
            0,
        )
        .ok_or(Error::ParseThoughtId {
            invalid_thought_id: value.to_string(),
        })?;

        let first_word = raw_first_word.to_string();

        let second_word = raw_second_word.to_string();

        Ok(Self {
            date_time,
            first_word,
            second_word,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn parse_test() {
        let expected = ThoughtId {
            date_time: Utc.with_ymd_and_hms(2024, 10, 10, 0, 0, 0).unwrap(),
            first_word: "dream".to_string(),
            second_word: "eject".to_string(),
        };

        let actual = "1728518400-dream-eject".parse().unwrap();

        assert_eq!(expected, actual);
    }
}
