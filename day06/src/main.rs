mod parser;

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use parser::Parser;

// https://adventofcode.com/2022/day/6
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut parser = Parser::new(4);

    for line in reader.lines() {
        let line = line?;
        parser.parse(&line)?;
    }

    println!("part 1: {}", parser.get_total());

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut parser = Parser::new(14);

    for line in reader.lines() {
        let line = line?;
        parser.parse(&line)?;
    }

    println!("part 2: {}", parser.get_total());

    Ok(())
}
