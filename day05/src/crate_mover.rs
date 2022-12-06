use std::{
    collections::VecDeque,
    fmt::Display,
    io::{BufRead, BufReader, Read},
    marker::PhantomData,
};

use anyhow::Context;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

pub struct CrateMover9000;
pub struct CrateMover9001;

pub struct CrateMover<M> {
    model: PhantomData<M>,
    stacks: Vec<VecDeque<char>>,
    moves: VecDeque<Move>,
}

impl<M> CrateMover<M> {
    pub fn new<R: Read>(buffer: BufReader<R>) -> anyhow::Result<Self> {
        let mut lines = buffer.lines();
        let mut line = lines.next().context("No lines in input")??;

        let stacks_count = (line.chars().count() + 1) / 4;

        let mut stacks = Vec::with_capacity(stacks_count);

        for _ in 0..stacks_count {
            stacks.push(VecDeque::new());
        }

        let re = Regex::new(r"[\[\s]([A-Z\s]{1})[\]\s]\s?")?;

        while line.contains('[') {
            for (i, cap) in re.captures_iter(&line).enumerate() {
                // cap[0] is the full match while cap[1] is the first capture group
                let mut c = cap[1].trim().chars();

                if let Some(c) = c.next() {
                    stacks[i].push_back(c);
                }
            }

            line = lines.next().context("No lines in input")??;
        }

        // empty line
        lines.next().context("No lines in input")??;

        let mut moves = VecDeque::new();

        for line in lines {
            let line = line?;
            let parts = line.split_whitespace().collect::<Vec<_>>();

            let amount = parts[1].parse::<usize>()?;
            let from = parts[3].parse::<usize>()? - 1;
            let to = parts[5].parse::<usize>()? - 1;

            moves.push_back(Move { amount, from, to });
        }

        Ok(Self {
            model: PhantomData,
            stacks,
            moves,
        })
    }
}

impl CrateMover<CrateMover9000> {
    pub fn move_crates(&mut self) -> anyhow::Result<()> {
        while let Some(m) = self.moves.pop_front() {
            for _ in 0..m.amount {
                let c = self.stacks[m.from]
                    .pop_front()
                    .context("No crates to move")?;

                self.stacks[m.to].push_front(c);
            }
        }

        Ok(())
    }
}

impl CrateMover<CrateMover9001> {
    pub fn move_crates(&mut self) -> anyhow::Result<()> {
        while let Some(m) = self.moves.pop_front() {
            let mut buffer = Vec::with_capacity(m.amount);

            for _ in 0..m.amount {
                let c = self.stacks[m.from]
                    .pop_front()
                    .context("No crates to move")?;

                buffer.push(c);
            }

            while let Some(c) = buffer.pop() {
                self.stacks[m.to].push_front(c);
            }
        }

        Ok(())
    }
}

impl<M> Display for CrateMover<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stack in &self.stacks {
            let c = stack.iter().next().unwrap_or(&' ');
            write!(f, "{c}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use std::io::BufReader;

    use super::*;

    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn test_input_parsing() {
        let reader = BufReader::new(INPUT.as_bytes());
        let crane = CrateMover::<CrateMover9000>::new(reader).unwrap();

        assert_eq!(crane.stacks.len(), 3);

        let mut stack_0 = crane.stacks[0].iter();
        assert_eq!(stack_0.next(), Some(&'N'));
        assert_eq!(stack_0.next(), Some(&'Z'));
        assert_eq!(stack_0.next(), None);

        let mut stack_1 = crane.stacks[1].iter();
        assert_eq!(stack_1.next(), Some(&'D'));
        assert_eq!(stack_1.next(), Some(&'C'));
        assert_eq!(stack_1.next(), Some(&'M'));
        assert_eq!(stack_1.next(), None);

        let mut stack_2 = crane.stacks[2].iter();
        assert_eq!(stack_2.next(), Some(&'P'));
        assert_eq!(stack_2.next(), None);

        assert_eq!(crane.moves.len(), 4);
        assert_eq!(
            crane.moves[0],
            Move {
                amount: 1,
                from: 1,
                to: 0
            }
        );
        assert_eq!(
            crane.moves[1],
            Move {
                amount: 3,
                from: 0,
                to: 2
            }
        );
        assert_eq!(
            crane.moves[2],
            Move {
                amount: 2,
                from: 1,
                to: 0
            }
        );
        assert_eq!(
            crane.moves[3],
            Move {
                amount: 1,
                from: 0,
                to: 1
            }
        );

        assert_eq!(crane.to_string(), "NDP");
    }

    #[test]
    fn test_crate_mover_9000() {
        let reader = BufReader::new(INPUT.as_bytes());

        let mut crane = CrateMover::<CrateMover9000>::new(reader).unwrap();
        crane.move_crates().unwrap();

        assert_eq!(crane.to_string(), "CMZ");
    }

    #[test]
    fn test_crate_mover_9001() {
        let reader = BufReader::new(INPUT.as_bytes());

        let mut crane = CrateMover::<CrateMover9001>::new(reader).unwrap();
        crane.move_crates().unwrap();

        assert_eq!(crane.to_string(), "MCD");
    }
}
