#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::io::BufRead;


fn read_input() -> Result<Vec<i32>, Error> {
    let mut input: Vec<i32> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut row: Vec<i32> = line?.split_whitespace()
            .map(|word| word.parse())
            .collect::<Result<Vec<i32>, _>>()?;
        input.append(&mut row);
    }
    Ok(input)
}


fn run() -> Result<(), Error> {
    let mut program = read_input()?;

    let mut pc: i32 = 0;
    let mut steps: i32 = 0;

    while 0 <= pc && pc < (program.len() as i32) {
        let jump = program[pc as usize];
        program[pc as usize] = jump + 1;
        pc = pc + jump;
        steps = steps + 1;
    }

    println!("{}", steps);

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
