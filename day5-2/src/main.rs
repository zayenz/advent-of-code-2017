extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;


fn read_input() -> Result<Vec<i32>, Error> {
    let mut input: Vec<i32> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for word in line?.split_whitespace() {
            input.push(word.parse()?);
        }
    }
    Ok(input)
}


fn run() -> Result<(), Error> {
    let mut program = read_input()?;

    let mut pc: i32 = 0;
    let mut steps: i32 = 0;

    while 0 <= pc && pc < (program.len() as i32) {
        let jump = program[pc as usize];
        if jump >= 3 {
            program[pc as usize] = jump - 1;
        } else {
            program[pc as usize] = jump + 1;
        }

        pc += jump;
        steps += 1;
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
