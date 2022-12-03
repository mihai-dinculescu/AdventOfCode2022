use anyhow::Context;

use crate::compartment_parser::CHAR_SCORES;

pub struct SetParser {
    size: usize,
    lines: Vec<String>,
    score: u64,
}

impl SetParser {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            lines: Vec::with_capacity(size),
            score: 0,
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<(), anyhow::Error> {
        self.lines.push(input.to_string());

        if self.lines.len() == self.size {
            self.score_set()?;
        }

        Ok(())
    }

    fn score_set(&mut self) -> Result<(), anyhow::Error> {
        // iterate through all items in the first backpack
        let chars = self.lines[0].chars();

        for c in chars {
            if self.lines.iter().skip(1).all(|value| value.contains(c)) {
                // item is in all backpacks
                self.score += CHAR_SCORES
                    .get(&c)
                    .context(format!("Missing score for char {c}"))?;
                break;
            }
        }

        self.lines.clear();

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
        let mut parser = SetParser::new(3);

        for line in reader.lines() {
            let line = line.unwrap();
            parser.parse(&line).unwrap();
        }

        assert_eq!(parser.get_score(), 70);
    }
}
