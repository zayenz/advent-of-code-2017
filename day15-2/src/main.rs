#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(unused_imports)]
#![allow(needless_range_loop)]

#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;

extern crate aoc2017;
use aoc2017::UnionFind;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Generator {
    value: u64,
    factor: u64,
    multiple: u64,
}

impl Generator {
    fn new(value: u64, factor: u64, multiple: u64) -> Generator {
        Generator {
            value,
            factor,
            multiple,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.value = (self.value * self.factor) % 2_147_483_647;
            if self.value % self.multiple == 0 {
                return Some(self.value);
            }
        }
    }
}

fn get_number_from_line(line: &str) -> Result<u64, Error> {
    let number = line.split_whitespace().nth(4).unwrap().parse()?;
    Ok(number)
}

fn read_input() -> Result<(Generator, Generator), Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a_start = get_number_from_line(&lines.next().unwrap()?)?;
    let b_start = get_number_from_line(&lines.next().unwrap()?)?;

    const A_FACTOR: u64 = 16_807;
    const A_MULTIPLE: u64 = 4;
    const B_FACTOR: u64 = 48_271;
    const B_MULTIPLE: u64 = 8;

    let a = Generator::new(a_start, A_FACTOR, A_MULTIPLE);
    let b = Generator::new(b_start, B_FACTOR, B_MULTIPLE);

    Ok((a, b))
}

fn matches(a: u64, b: u64) -> bool {
    (a & 0xFFFF) == (b & 0xFFFF)
}

fn run() -> Result<(), Error> {
    let (a, b) = read_input()?;

    let matches_count = a.zip(b)
        .take(5_000_000)
        .filter(|&(a, b)| matches(a, b))
        .count();

    println!("{}", matches_count);

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
