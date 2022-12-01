use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

mod parser;

use parser::Parser;

// https://adventofcode.com/2022/day/1
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut parser_top_1 = Parser::new(1);
    let mut parser_top_3 = Parser::new(3);

    for line in reader.lines() {
        let line = line?;
        parser_top_1.parse(&line)?;
        parser_top_3.parse(&line)?;
    }
    println!("part 1: {}", parser_top_1.get_max());
    println!("part 2: {}", parser_top_3.get_max());

    Ok(())
}
