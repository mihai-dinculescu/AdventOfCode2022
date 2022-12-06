mod crate_mover;

use std::{error::Error, fs::File, io::BufReader};

use crate_mover::{CrateMover, CrateMover9000, CrateMover9001};

// https://adventofcode.com/2022/day/5
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut crane = CrateMover::<CrateMover9000>::new(reader)?;
    crane.move_crates()?;

    println!("Part 1: {}", crane);

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut crane = CrateMover::<CrateMover9001>::new(reader)?;
    crane.move_crates()?;

    println!("Part 2: {}", crane);

    Ok(())
}
