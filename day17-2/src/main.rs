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
    let values = 50_000_000;

    let mut position: usize = 1;
    let mut value_count = 2;
    let mut cell_after_0 = 1;

    for i in 2..values {
        position = (position + steps) % value_count;
        value_count += 1;
        if position == 0 {
            cell_after_0 = i;
        }
        position += 1;
    }

    println!("{}", cell_after_0);

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
