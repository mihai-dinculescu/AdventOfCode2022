use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use parser::Pair;

mod parser;

// https://adventofcode.com/2022/day/4
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut count_part_1 = 0;
    let mut count_part_2 = 0;

    for line in reader.lines() {
        let line = line?;
        let pair = Pair::new(&line)?;

        if pair.overlaps() {
            count_part_2 += 1;

            if pair.contains() {
                count_part_1 += 1;
            }
        }
    }

    println!("part 1: {count_part_1}");
    println!("part 2: {count_part_2}");

    Ok(())
}
