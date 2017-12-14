#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;
use std::collections::HashMap;


fn read_input() -> Result<String, Error> {
    let mut input: HashMap<i32, i32> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            return Ok(line.to_string());
        }
    }
    bail!("No input")
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

    fn hash_string(&self) -> String {
        let hash = self.hash();
        let mut result = String::new();
        for hash_part in hash {
            result += &format!("{:02x}", hash_part);
        }
        result
    }

    fn hash_count_ones(&self) -> u32 {
        let mut result = 0;
        for hash_part in self.hash() {
            result += hash_part.count_ones();
        }
        return result;
    }

    fn hash(self) -> Vec<u8> {
        let hash: Vec<u8> = self.contents
            .chunks(16)
            .map(|chunk| chunk.iter().fold(0u8, |a, b| a ^ b))
            .collect::<Vec<_>>();
        hash
    }
}

fn hash_count(s: String) -> u32 {
    let mut region_widths: Vec<usize> = Vec::new();
    for ch in s.trim().chars() {
        region_widths.push(ch as usize)
    }
    region_widths.append(&mut vec![17, 31, 73, 47, 23]);
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

    memory.hash_count_ones()
}

fn run() -> Result<(), Error> {
    let base_key = read_input()?;

    let mut ones = 0;
    for row in 0..128 {
        let row_key = format!("{}-{}", base_key, row);
        let row_hash_count = hash_count(row_key);
        ones += row_hash_count;
    }

    println!("{}", ones);

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
