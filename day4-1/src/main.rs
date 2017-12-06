extern crate failure;
use failure::Error;

use std::{io, process};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::io::BufRead;


fn read_input() -> Result<Vec<Vec<String>>, Error> {
    let mut input: Vec<Vec<String>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let row: Vec<String> = line?.split_whitespace().map(|word| word.into()).collect();
        if !row.is_empty() {
            input.push(row);
        }
    }
    Ok(input)
}

fn checksum_row(row: &[String]) -> Result<u32, Error> {
    let words: HashSet<&String> = HashSet::from_iter(row.iter());
    if words.len() == row.len() {
        Ok(1)
    } else {
        Ok(0)
    }

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
