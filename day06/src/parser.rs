use std::{collections::HashMap, error::Error};

pub struct Parser {
    marker_length: usize,
    total: usize,
}

impl Parser {
    pub fn new(marker_length: usize) -> Self {
        Parser {
            marker_length,
            total: 0,
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<usize, Box<dyn Error>> {
        let mut memory = HashMap::new();

        for (i, c) in input.chars().enumerate() {
            memory.insert(c, i);

            if memory.len() >= self.marker_length {
                let index_to_keep = i - self.marker_length + 1;
                memory.retain(|_, ri| *ri >= index_to_keep);
            }

            if memory.len() >= self.marker_length {
                self.total += i + 1;
                return Ok(i + 1);
            }
        }

        Ok(usize::MAX)
    }

    pub fn get_total(&self) -> usize {
        self.total
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let mut parser = Parser::new(4);

        assert_eq!(parser.parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 7);
        assert_eq!(parser.parse("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(parser.parse("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(
            parser.parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
            10
        );
        assert_eq!(
            parser.parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
            11
        );

        assert_eq!(parser.get_total(), 39);
    }
}
