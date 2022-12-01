use std::cmp::Reverse;
use std::{collections::BinaryHeap, error::Error};

pub struct Parser {
    top_n: usize,
    maximums: BinaryHeap<Reverse<u64>>,
    current: u64,
}

impl Parser {
    pub fn new(top_n: usize) -> Self {
        Self {
            top_n,
            maximums: BinaryHeap::with_capacity(top_n),
            current: 0,
        }
    }

    pub fn parse(&mut self, value: &str) -> Result<(), Box<dyn Error>> {
        match value.is_empty() {
            true => {
                if self.current > self.maximums.peek().unwrap_or(&Reverse(0)).0 {
                    // we've got a new maximum
                    if self.maximums.len() >= self.top_n {
                        // we already have enough maximums, so we remove the smallest one
                        // the smallest one is at the top of the heap
                        self.maximums.pop();
                    }

                    // we push the reverse value so that we can pop the smallest one
                    self.maximums.push(Reverse(self.current));
                }

                self.current = 0;
            }
            false => {
                let calories = value.parse::<u64>()?;
                self.current += calories;
            }
        }

        Ok(())
    }

    pub fn get_max(&self) -> u64 {
        self.maximums
            .iter()
            .map(|Reverse(calories)| *calories)
            .sum::<u64>()
    }
}

#[cfg(test)]
pub mod tests {
    use std::io::{BufRead, BufReader};

    use super::*;

    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

"#;

    #[test]
    fn test_parser_top_1() {
        let reader = BufReader::new(INPUT.as_bytes());
        let mut parser = Parser::new(1);

        for line in reader.lines() {
            let line = line.unwrap();
            parser.parse(&line).unwrap();
        }

        assert_eq!(parser.get_max(), 24000);
    }

    #[test]
    fn test_parser_top_3() {
        let reader = BufReader::new(INPUT.as_bytes());
        let mut parser = Parser::new(3);

        for line in reader.lines() {
            let line = line.unwrap();
            parser.parse(&line).unwrap();
        }

        assert_eq!(parser.get_max(), 45000);
    }
}
