extern crate failure;
use failure::Error;

use std::{io, process};
use std::collections::HashSet;
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

fn redistribute(memory: Vec<i32>) -> Vec<i32> {
    let mut result = memory;

    let mut index: usize = 0;
    let mut banks_left: i32 = 0;
    for (i, &v) in result.iter().enumerate() {
        if v > banks_left {
            index = i;
            banks_left = v;
        }
    }

    result[index] = 0;

    while banks_left > 0 {
        index = (index + 1) % result.len();
        result[index] += 1;
        banks_left -= 1;
    }

    result
}

fn run() -> Result<(), Error> {
    let mut memory = read_input()?;

    let mut steps: i32 = 0;
    let mut seen = HashSet::new();
    seen.insert(memory.clone());

    loop {
        memory = redistribute(memory);
        steps += 1;

        if seen.contains(&memory) {
            println!("{}", steps);
            return Ok(());
        }

        seen.insert(memory.clone());
    }
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
