use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

mod compartment_parser;
mod set_parser;

use compartment_parser::CompartmentParser;
use set_parser::SetParser;

// https://adventofcode.com/2022/day/3
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut compartment_parser = CompartmentParser::new();
    let mut set_parser = SetParser::new(3);

    for line in reader.lines() {
        let line = line?;
        compartment_parser.parse(&line)?;
        set_parser.parse(&line)?;
    }

    println!("part 1: {}", compartment_parser.get_score());
    println!("part 2: {}", set_parser.get_score());

    Ok(())
}
