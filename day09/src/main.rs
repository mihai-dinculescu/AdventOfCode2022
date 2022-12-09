mod parser;

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

// https://adventofcode.com/2022/day/9
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut parser_2 = parser::Parser::new(2);
    let mut parser_10 = parser::Parser::new(10);

    for line in reader.lines() {
        let line = line?;
        parser_2.move_head(&line)?;
        parser_10.move_head(&line)?;
    }

    println!("part 1: {}", parser_2.get_tail_visited_positions());
    println!("part 2: {}", parser_10.get_tail_visited_positions());

    Ok(())
}
