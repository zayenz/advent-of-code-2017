#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;


fn read_input() -> Result<Vec<Vec<u32>>, Error> {
    let mut input: Vec<Vec<u32>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let row: Vec<u32> = line?.split_whitespace()
            .flat_map(|cell| cell.parse())
            .collect();
        if !row.is_empty() {
            input.push(row);
        }
    }
    Ok(input)
}

fn checksum_row(row: &[u32]) -> Result<u32, Error> {
    for a in row {
        for b in row {
            if a != b {
                let d = a / b;
                if d * b == *a {
                    return Ok(d);
                }
            }
        }
    }
    bail!("No division found in row {:?}", row)
}

fn run() -> Result<(), Error> {
    let input = read_input()?;
    let row_checksums = input
        .iter()
        .map(|row| checksum_row(row))
        .collect::<Result<Vec<_>, _>>()?;

    let checksum: u32 = row_checksums.iter().sum();

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
