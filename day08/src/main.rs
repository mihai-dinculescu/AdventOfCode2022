mod parser;

use std::{error::Error, fs::File, io::BufReader};

use parser::Parser;

// https://adventofcode.com/2022/day/8
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let parser = Parser::new(reader)?;

    let (visible_trees, max_scenic_score) = parser.evaluate()?;
    println!("part 1: {visible_trees}");
    println!("part 2: {max_scenic_score}");

    Ok(())
}
