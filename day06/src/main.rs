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

    let mut parser_part_1 = Parser::new(4);
    let mut parser_part_2 = Parser::new(14);

    for line in reader.lines() {
        let line = line?;
        parser_part_1.parse(&line)?;
        parser_part_2.parse(&line)?;
    }

    println!("part 1: {}", parser_part_1.get_total());
    println!("part 2: {}", parser_part_2.get_total());

    Ok(())
}
