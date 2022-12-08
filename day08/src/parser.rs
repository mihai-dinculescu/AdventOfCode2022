use std::io::{BufRead, BufReader, Read};

use anyhow::Context;

pub struct Parser {
    rows: usize,
    columns: usize,
    trees: Vec<u32>,
}

impl Parser {
    pub fn new<R: Read>(buffer: BufReader<R>) -> anyhow::Result<Self> {
        let mut rows = 0;
        let mut columns = 0;

        let mut trees = Vec::new();

        for row in buffer.lines() {
            let row = row?;

            let row_digits = row
                .chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<_>>>()
                .context(format!("failed to parse the digits in row {rows}"))?;

            if columns == 0 {
                columns = row_digits.len();
            } else if columns != row_digits.len() {
                return Err(anyhow::anyhow!("invalid row length for row {rows}"));
            }

            trees.extend(row_digits);

            rows += 1;
        }

        Ok(Self {
            rows,
            columns,
            trees,
        })
    }

    pub fn evaluate(&self) -> anyhow::Result<(usize, u32)> {
        let mut visible_trees = 0;
        let mut max_scenic_score = 0;

        // add the edges
        visible_trees += self.rows * 2 + (self.columns - 2) * 2;

        for row in 1..self.rows - 1 {
            for column in 1..self.columns - 1 {
                let tree = self
                    .get_tree_index(row, column)
                    .context("failed to get tree index")?;

                if tree == &0 {
                    // there's no scenario in which this can be visible
                    continue;
                }

                // we get the trees in each direction
                let top_trees = (0..row)
                    .rev()
                    .map(|r| self.get_tree_index(r, column).unwrap_or(&u32::MAX));
                let bottom_trees = (row + 1..self.rows)
                    .map(|r| self.get_tree_index(r, column).unwrap_or(&u32::MAX));
                let left_trees = (0..column)
                    .rev()
                    .map(|c| self.get_tree_index(row, c).unwrap_or(&u32::MAX));
                let right_trees = (column + 1..self.columns)
                    .map(|c| self.get_tree_index(row, c).unwrap_or(&u32::MAX));

                let fold_func = |(total_trees, trees_can_view, blocked), t| {
                    if blocked {
                        (total_trees + 1, trees_can_view, blocked)
                    } else {
                        (total_trees + 1, trees_can_view + 1, t >= tree)
                    }
                };

                // we fold the trees in each direction to get the number of trees that can be seen and the blocked status
                let (_, top_trees_can_view, top_blocked) =
                    top_trees.fold((0u32, 0u32, false), fold_func);
                let (_, bottom_trees_can_view, bottom_blocked) =
                    bottom_trees.fold((0u32, 0u32, false), fold_func);
                let (_, left_trees_can_view, left_blocked) =
                    left_trees.fold((0u32, 0u32, false), fold_func);
                let (_, right_trees_can_view, right_blocked) =
                    right_trees.fold((0u32, 0u32, false), fold_func);

                if !top_blocked || !bottom_blocked || !left_blocked || !right_blocked {
                    visible_trees += 1;
                }

                let scenic_score = top_trees_can_view
                    * bottom_trees_can_view
                    * left_trees_can_view
                    * right_trees_can_view;

                if scenic_score > max_scenic_score {
                    max_scenic_score = scenic_score;
                }
            }
        }

        Ok((visible_trees, max_scenic_score))
    }

    fn get_tree_index(&self, row: usize, column: usize) -> Option<&u32> {
        self.trees.get(row * self.columns + column)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &str = r#"30373
25512
65332
33549
35390
"#;

    #[test]
    fn test_parser() {
        let reader = BufReader::new(INPUT.as_bytes());
        let parser = Parser::new(reader).unwrap();

        assert_eq!(parser.rows, 5);
        assert_eq!(parser.columns, 5);

        assert_eq!(parser.trees.len(), parser.rows * parser.columns);

        // thank you copilot for generating these
        assert_eq!(parser.get_tree_index(0, 0).unwrap(), &3);
        assert_eq!(parser.get_tree_index(0, 1).unwrap(), &0);
        assert_eq!(parser.get_tree_index(0, 2).unwrap(), &3);
        assert_eq!(parser.get_tree_index(0, 3).unwrap(), &7);
        assert_eq!(parser.get_tree_index(0, 4).unwrap(), &3);

        assert_eq!(parser.get_tree_index(1, 0).unwrap(), &2);
        assert_eq!(parser.get_tree_index(1, 1).unwrap(), &5);
        assert_eq!(parser.get_tree_index(1, 2).unwrap(), &5);
        assert_eq!(parser.get_tree_index(1, 3).unwrap(), &1);
        assert_eq!(parser.get_tree_index(1, 4).unwrap(), &2);

        assert_eq!(parser.get_tree_index(2, 0).unwrap(), &6);
        assert_eq!(parser.get_tree_index(2, 1).unwrap(), &5);
        assert_eq!(parser.get_tree_index(2, 2).unwrap(), &3);
        assert_eq!(parser.get_tree_index(2, 3).unwrap(), &3);
        assert_eq!(parser.get_tree_index(2, 4).unwrap(), &2);

        assert_eq!(parser.get_tree_index(3, 0).unwrap(), &3);
        assert_eq!(parser.get_tree_index(3, 1).unwrap(), &3);
        assert_eq!(parser.get_tree_index(3, 2).unwrap(), &5);
        assert_eq!(parser.get_tree_index(3, 3).unwrap(), &4);
        assert_eq!(parser.get_tree_index(3, 4).unwrap(), &9);

        assert_eq!(parser.get_tree_index(4, 0).unwrap(), &3);
        assert_eq!(parser.get_tree_index(4, 1).unwrap(), &5);
        assert_eq!(parser.get_tree_index(4, 2).unwrap(), &3);
        assert_eq!(parser.get_tree_index(4, 3).unwrap(), &9);
        assert_eq!(parser.get_tree_index(4, 4).unwrap(), &0);
    }

    #[test]
    fn test_evaluate() {
        let reader = BufReader::new(INPUT.as_bytes());
        let parser = Parser::new(reader).unwrap();

        assert_eq!(parser.evaluate().unwrap(), (21, 8));
    }
}
