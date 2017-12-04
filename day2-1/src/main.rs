#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;


fn read_input() -> Result<Vec<Vec<u32>>, Error> {
    let mut input: Vec<Vec<u32>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let row = line?.split_whitespace()
            .flat_map(|cell| cell.parse())
            .collect();
        input.push(row);
    }
    Ok(input)
}

fn checksum_row(row: &Vec<u32>) -> Option<u32> {
    let max = row.iter().max()?;
    let min = row.iter().min()?;
    Some(max - min)
}

fn run() -> Result<(), Error> {
    let input = read_input()?;

    let checksum: u32 = input.iter().flat_map(|row| checksum_row(row)).sum();

    println!("{}", checksum);

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
