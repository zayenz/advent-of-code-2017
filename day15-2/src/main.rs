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

fn get_number_from_line(line: &str) -> Result<u64, Error> {
    let number = line.split_whitespace().nth(4).unwrap().parse()?;
    Ok(number)
}

fn read_input() -> Result<(u64, u64), Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = get_number_from_line(&lines.next().unwrap()?)?;
    let b = get_number_from_line(&lines.next().unwrap()?)?;
    Ok((a, b))
}

fn next(v: u64, factor: u64, multiple: u64) -> u64 {
    const A_FACTOR: u64 = 16_807;
    let mut value = v;
    loop {
        value = (value * factor) % 2_147_483_647;
        if value % multiple == 0 {
            return value;
        }
    }

}

fn next_a(v: u64) -> u64 {
    const A_FACTOR: u64 = 16_807;
    const A_MULTIPLE: u64 = 4;
    next(v, A_FACTOR, A_MULTIPLE)
}

fn next_b(v: u64) -> u64 {
    const B_FACTOR: u64 = 48_271;
    const B_MULTIPLE: u64 = 8;
    next(v, B_FACTOR, B_MULTIPLE)
}


fn matches(a: u64, b: u64) -> bool {
    (a & 0xFFFF) == (b & 0xFFFF)
}


fn run() -> Result<(), Error> {
    let (mut a, mut b) = read_input()?;

    let mut matches_count = 0;

    for _ in 0..5_000_000 {
        a = next_a(a);
        b = next_b(b);
        if matches(a, b) {
            matches_count += 1;
        }
    }

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
