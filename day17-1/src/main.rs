extern crate failure;
use failure::Error;

use std::{io, process};


fn read_input() -> Result<usize, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}


fn run() -> Result<(), Error> {
    let steps = read_input()?;

    let mut buffer: Vec<usize> = Vec::with_capacity(2018);
    buffer.push(0);
    let mut position: usize = 0;

    for i in 1..2018 {
        position = (position + steps) % buffer.len();
        buffer.insert(position + 1, i);
        position += 1;
    }

    let final_cell_value = buffer[(position + 1) % buffer.len()];

    println!("{}", final_cell_value);

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
