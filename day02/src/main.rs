use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

mod strategy1;
mod strategy2;

use crate::{strategy1::Strategy1, strategy2::Strategy2};

// https://adventofcode.com/2022/day/2
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut score_part_1 = 0;
    let mut score_part_2 = 0;

    for line in reader.lines() {
        let line = line?;

        let game = Strategy1::new(&line)?;
        score_part_1 += game.score();

        let game = Strategy2::new(&line)?;
        score_part_2 += game.score();
    }

    println!("part 1: {score_part_1}");
    println!("part 2: {score_part_2}");

    Ok(())
}
