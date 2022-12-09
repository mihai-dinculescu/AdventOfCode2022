use std::collections::HashSet;

use anyhow::Context;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Segment {
    x: i32,
    y: i32,
}

pub struct Parser {
    segments: Vec<Segment>,
    tail_visited_positions: HashSet<Segment>,
}

impl Segment {
    fn new() -> Self {
        Segment { x: 0, y: 0 }
    }
}

impl Parser {
    pub fn new(segments_no: usize) -> Self {
        let mut segments = Vec::with_capacity(segments_no);

        for _ in 0..segments_no {
            segments.push(Segment::new());
        }

        Parser {
            segments,
            tail_visited_positions: HashSet::from([Segment::new()]),
        }
    }

    pub fn move_head(&mut self, direction: &str) -> anyhow::Result<()> {
        let mut parts = direction.split(' ');
        let direction = parts.next().context("missing direction")?;
        let distance = parts.next().context("missing distance")?.parse::<i32>()?;

        let (move_x, move_y) = match direction {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => anyhow::bail!("invalid direction"),
        };

        for _ in 0..distance {
            let head = self
                .segments
                .get_mut(0)
                .context("failed to get segment at index 0")?;

            head.x += move_x;
            head.y += move_y;

            for i in 1..self.segments.len() {
                self.move_segment(i)?;
            }
        }

        Ok(())
    }

    pub fn get_tail_visited_positions(&self) -> usize {
        self.tail_visited_positions.len()
    }

    fn move_segment(&mut self, index: usize) -> anyhow::Result<()> {
        let tail_index = self.segments.len() - 1;

        let prev_segment = self
            .segments
            .get(index - 1)
            .context("failed to get the previous segment")?
            .clone();

        let curr_segment = self
            .segments
            .get_mut(index)
            .context("failed to get the current segment")?;

        if prev_segment.x == curr_segment.x {
            // x is same, y is different
            if prev_segment.y > curr_segment.y + 1 {
                curr_segment.y += 1;
            } else if prev_segment.y < curr_segment.y - 1 {
                curr_segment.y -= 1;
            } else {
                // they are close enough, nothing to do
                return Ok(());
            }
        } else if prev_segment.y == curr_segment.y {
            // y is same, x is different
            if prev_segment.x > curr_segment.x + 1 {
                curr_segment.x += 1;
            } else if prev_segment.x < curr_segment.x - 1 {
                curr_segment.x -= 1;
            } else {
                // they are close enough, nothing to do
                return Ok(());
            }
        } else {
            // x and y are different
            if (prev_segment.x - curr_segment.x).abs() + (prev_segment.y - curr_segment.y).abs() > 2
            {
                // they are far away, move closer
                curr_segment.x += match prev_segment.x.cmp(&curr_segment.x) {
                    std::cmp::Ordering::Greater => 1,
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                };

                curr_segment.y += match prev_segment.y.cmp(&curr_segment.y) {
                    std::cmp::Ordering::Greater => 1,
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                };
            } else {
                // they are close enough, nothing to do
                return Ok(());
            }
        }

        // if we've got here without returning it means that the segment has moved
        if index == tail_index {
            self.tail_visited_positions.insert(curr_segment.clone());
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use std::io::{BufRead, BufReader};

    use super::*;

    const INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    #[test]
    fn test_parser_with_2_segments() {
        let mut parser = Parser::new(2);
        let reader = BufReader::new(INPUT.as_bytes());

        for line in reader.lines() {
            let line = line.unwrap();
            parser.move_head(&line).unwrap();
        }

        assert_eq!(parser.get_tail_visited_positions(), 13);
    }

    #[test]
    fn test_parser_with_10_segments() {
        let mut parser = Parser::new(10);
        let reader = BufReader::new(INPUT.as_bytes());

        for line in reader.lines() {
            let line = line.unwrap();
            parser.move_head(&line).unwrap();
        }

        assert_eq!(parser.get_tail_visited_positions(), 1);
    }
}
