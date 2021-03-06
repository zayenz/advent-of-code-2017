extern crate failure;
use failure::Error;

use std::{io, process};

fn run() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let nums: Vec<u32> = input.chars().flat_map(|c| c.to_digit(10)).collect();
    let pairs = nums.iter().zip(nums.iter().cycle().skip(1));
    let equal_pairs = pairs.filter(|&(a, b)| a == b);

    let result: u32 = equal_pairs.map(|(a, _b)| a).sum();

    println!("{}", result);

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => process::exit(0),
        Err(error) => {
            for cause in error.causes() {
                eprintln!("{}", cause)
            }
            process::exit(1)
        }
    }
}
