extern crate failure;
use failure::Error;

use std::{io, process};
use std::collections::HashSet;
use std::io::BufRead;


fn read_input() -> Result<(usize, Vec<usize>), Error> {
    let mut input: Vec<usize> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for word in line?.split_whitespace() {
            input.push(word.parse()?);
        }
    }

    let length = input.remove(0);
    Ok((length, input))
}

const MAX_LENGTH: usize = 256;

#[derive(Copy, Clone)]
struct Memory {
    contents: [u8; MAX_LENGTH],
    length: usize,
}

impl Memory {
    fn new(length: usize) -> Memory {
        assert!(length > 1, "At least two elements required");
        assert!(length <= MAX_LENGTH, "At most MAX_LENGTH is supported");
        let mut result = Memory {
            contents: [0; MAX_LENGTH],
            length
        };
        for i in 0..MAX_LENGTH {
            result.contents[i] = i as u8;
        };
        result
    }

    fn reverse(&mut self, from: usize, width: usize) {
        if width > 1 {
            for i in 0..(width / 2) {
                self.contents.swap((from + i) % self.length, (from + width - 1 - i) % self.length);
            }
        }
    }

    fn hash_value(&self) -> usize {
        self.contents[0] as usize * self.contents[1] as usize
    }
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.length {
            write!(f, "{},", self.contents[i as usize])?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

fn run() -> Result<(), Error> {
    let (length, region_widths) = read_input()?;

    let mut memory = Memory::new(length);

    let mut position = 0;
    let mut skip_length = 0;
    for region_width in region_widths {
        memory.reverse(position, region_width);
        position += region_width + skip_length;
        skip_length += 1;
    }

    let hash = memory.hash_value();
    println!("{}", hash);

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
