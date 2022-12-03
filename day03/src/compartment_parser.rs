use std::collections::HashMap;

use anyhow::Context;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CHAR_SCORES: HashMap<char, u64> = {
        let mut char_scores = HashMap::with_capacity(52);

        for (i, c) in (1..=26).zip('a'..='z') {
            char_scores.insert(c, i);
        }
        // probably it's faster to iterate a second time rather than `.to_uppercase()`
        for (i, c) in (27..=52).zip('A'..='Z') {
            char_scores.insert(c, i);
        }

        char_scores
    };
}

pub struct CompartmentParser {
    score: u64,
}

impl CompartmentParser {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn parse(&mut self, input: &str) -> Result<(), anyhow::Error> {
        let middle = input.chars().count() / 2;

        let iter_start = input.chars().take(middle);
        let iter_end = input.chars().skip(middle);

        for cs in iter_start {
            if iter_end.clone().any(|ce| ce == cs) {
                self.score += CHAR_SCORES
                    .get(&cs)
                    .context(format!("Missing score for char {cs}"))?;
                break;
            }
        }

        Ok(())
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }
}

#[cfg(test)]
pub mod tests {
    use std::io::{BufRead, BufReader};

    use super::*;

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn it_works() {
        let reader = BufReader::new(INPUT.as_bytes());
        let mut parser = CompartmentParser::new();

        for line in reader.lines() {
            let line = line.unwrap();
            parser.parse(&line).unwrap();
        }

        assert_eq!(parser.get_score(), 157);
    }
}
