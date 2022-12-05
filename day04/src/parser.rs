use anyhow::Context;

pub struct Pair {
    first: Assignment,
    second: Assignment,
}

impl Pair {
    pub fn new(input: &str) -> Result<Self, anyhow::Error> {
        let mut parts = input.split(',');
        let first = parts.next().context("missing first assignment")?;
        let second = parts.next().context("missing second assignment")?;

        Ok(Self {
            first: Assignment::new(first)?,
            second: Assignment::new(second)?,
        })
    }

    pub fn contains(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    pub fn overlaps(&self) -> bool {
        self.first.overlaps(&self.second) || self.second.overlaps(&self.first)
    }
}

pub struct Assignment {
    start: u64,
    end: u64,
}

impl Assignment {
    pub fn new(input: &str) -> Result<Self, anyhow::Error> {
        let mut parts = input.split('-');
        let start = parts.next().context("missing start")?;
        let end = parts.next().context("missing end")?;

        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }

    pub fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

#[cfg(test)]
pub mod tests {
    use std::io::{BufRead, BufReader};

    use super::*;

    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn test_contains_false() {
        let input = "2-4,6-8";
        let pair = Pair::new(input).unwrap();
        assert!(!pair.contains());

        let input = "2-3,4-5";
        let pair = Pair::new(input).unwrap();
        assert!(!pair.contains());

        let input = "5-7,7-9";
        let pair = Pair::new(input).unwrap();
        assert!(!pair.contains());

        let input = "2-6,4-8";
        let pair = Pair::new(input).unwrap();
        assert!(!pair.contains());
    }

    #[test]
    fn test_contains_true() {
        let input = "2-8,3-7";
        let pair = Pair::new(input).unwrap();
        assert!(pair.contains());

        let input = "6-6,4-6";
        let pair = Pair::new(input).unwrap();
        assert!(pair.contains());
    }

    #[test]
    fn test_contains_all() {
        let reader = BufReader::new(INPUT.as_bytes());
        let mut count = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            let pair = Pair::new(&line).unwrap();
            if pair.contains() {
                count += 1;
            }
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn test_overlaps_false() {
        let input = "2-4,6-8";
        let pair = Pair::new(input).unwrap();
        assert!(!pair.overlaps());

        let input = "2-3,4-5";
        let pair = Pair::new(input).unwrap();
        assert!(!pair.overlaps());
    }

    #[test]
    fn test_overlaps_true() {
        let input = "5-7,7-9";
        let pair = Pair::new(input).unwrap();
        assert!(pair.overlaps());

        let input = "2-8,3-7";
        let pair = Pair::new(input).unwrap();
        assert!(pair.overlaps());

        let input = "6-6,4-6";
        let pair = Pair::new(input).unwrap();
        assert!(pair.overlaps());

        let input = "2-6,4-8";
        let pair = Pair::new(input).unwrap();
        assert!(pair.overlaps());
    }

    #[test]
    fn test_overlaps_all() {
        let reader = BufReader::new(INPUT.as_bytes());
        let mut count = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            let pair = Pair::new(&line).unwrap();
            if pair.overlaps() {
                count += 1;
            }
        }

        assert_eq!(count, 4);
    }
}
