mod parser;

use std::{error::Error, fs::File, io::BufReader};

use parser::Parser;

// https://adventofcode.com/2022/day/7
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let parser = Parser::new(reader)?;

    println!("part 1: {}", parser.get_all_by_top_limit(100000));

    println!(
        "part 2: {}",
        parser.get_one_by_free_space_required(70000000, 30000000)?
    );

    Ok(())
}
