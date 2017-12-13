#![allow(dead_code)]

#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;
use std::collections::HashMap;


fn read_input() -> Result<HashMap<i32, i32>, Error> {
    let mut input: HashMap<i32, i32> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut words = line.trim().split(": ");
        if let Some(layer) = words.next() {
            let depth = words.next().unwrap();
            input.insert(layer.parse()?, depth.parse()?);
        }
    }
    Ok(input)
}

/// Improved version that uses iterators to express intent of search better than
/// previous convoluted for-loops.
fn run() -> Result<(), Error> {
    let layers = read_input()?;

    let delay = (0..std::i32::MAX).find(|&delay| {
        layers.iter().all(|(layer, depth)| {
            (layer + delay) % (2 * (depth - 1)) != 0
        })
    });

    if let Some(delay) = delay {
        println!("{}", delay);
        Ok(())
    } else {
        bail!("No delay found up to {}", std::i32::MAX)
    }
}

/// This is the original version of the code used to submit the solution.
fn run_original() -> Result<(), Error> {
    let layers = read_input()?;
    let max_depth = layers.values().max().unwrap();
    let max_delay = max_depth * max_depth * max_depth * max_depth * max_depth;

    'delay_loop: for delay in 0..max_delay {
        for (&layer, &depth) in &layers {
            let caught = {
                (layer + delay) % (2 * (depth - 1)) == 0
            };
            if caught {
                continue 'delay_loop;
            }
        }
        println!("{}", delay);
        return Ok(());
    }

    bail!("No delay found up to {}", max_delay)
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
