extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;


fn read_input() -> Result<Vec<usize>, Error> {
    let mut input: Vec<usize> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for ch in line?.trim().chars() {
            input.push(ch as usize)
        }
    }

    input.append(&mut vec![17, 31, 73, 47, 23]);

    Ok(input)
}

const DATA_LENGTH: usize = 256;

#[derive(Copy, Clone)]
struct Memory {
    contents: [u8; DATA_LENGTH],
}

impl Memory {
    fn new() -> Memory {
        let mut result = Memory { contents: [0; DATA_LENGTH] };
        for i in 0..DATA_LENGTH {
            result.contents[i] = i as u8;
        }
        result
    }

    fn reverse(&mut self, from: usize, width: usize) {
        if width > 1 {
            for i in 0..(width / 2) {
                self.contents.swap(
                    (from + i) % DATA_LENGTH,
                    (from + width - 1 - i) % DATA_LENGTH,
                );
            }
        }
    }

    fn hash_value(&self) -> String {
        let hash = self.contents.chunks(16).map(|chunk| {
            chunk.iter().fold(0u8, |a, b| a ^ b)
        });
        let mut result = String::new();
        for hash_part in hash {
            result += &format!("{:02x}", hash_part);
        }
        result
    }
}

fn run() -> Result<(), Error> {
    let region_widths = read_input()?;

    let mut memory = Memory::new();

    let mut position = 0;
    let mut skip_length = 0;
    for _ in 0..64 {
        for region_width in &region_widths {
            memory.reverse(position, *region_width);
            position += region_width + skip_length;
            skip_length += 1;
        }
    }

    let hash = memory.hash_value();
    println!("{}", hash);

    Ok(())
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..DATA_LENGTH {
            write!(f, "{},", self.contents[i as usize])?;
        }
        write!(f, "]")?;
        Ok(())
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
