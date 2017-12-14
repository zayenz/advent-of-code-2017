#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(needless_range_loop)]

#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;

fn read_input() -> Result<String, Error> {
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

#[derive(Debug)]
struct UnionFind {
    nodes: Vec<i32>,
}

impl UnionFind {
    fn new(size: i32) -> UnionFind {
        let mut nodes = Vec::with_capacity(size as usize);
        for _ in 1..size + 1 {
            nodes.push(-1);
        }
        UnionFind { nodes }
    }

    fn find(&self, node: i32) -> i32 {
        assert!(0 <= node && (node as usize) < self.nodes.len());
        let root = self.nodes[node as usize];
        if root >= 0 { self.find(root) } else { node }
    }

    fn join(&mut self, node1: i32, node2: i32) {
        let root1 = self.find(node1);
        let root2 = self.find(node2);
        if root1 != root2 {
            self.nodes[root1 as usize] += self.nodes[root2 as usize];
            self.nodes[root2 as usize] = root1;
        }
    }

    fn size(&self, node: i32) -> i32 {
        let root = self.find(node);
        self.nodes[root as usize].abs()
    }

    fn group_count(&self) -> i32 {
        let mut result = 0;
        for &node in &self.nodes {
            if node < 0 {
                result += 1;
            }
        }
        result
    }
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
        result
    }

    fn hash(self) -> Vec<u8> {
        let hash: Vec<u8> = self.contents
            .chunks(16)
            .map(|chunk| chunk.iter().fold(0u8, |a, b| a ^ b))
            .collect::<Vec<_>>();
        hash
    }
}

fn hash(s: &str) -> Vec<bool> {
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

    let mut result: Vec<bool> = Vec::new();
    for hash_octet in memory.hash() {
        result.push((hash_octet & 0x80) != 0);
        result.push((hash_octet & 0x40) != 0);
        result.push((hash_octet & 0x20) != 0);
        result.push((hash_octet & 0x10) != 0);
        result.push((hash_octet & 0x08) != 0);
        result.push((hash_octet & 0x04) != 0);
        result.push((hash_octet & 0x02) != 0);
        result.push((hash_octet & 0x01) != 0);
    }
    result
}

fn run() -> Result<(), Error> {
    let base_key = read_input()?;

    // Construct grid
    //
    let mut grid = [[0; 128]; 128];
    let mut counter = 1;
    for row in 0..128 {
        let row_key = format!("{}-{}", base_key, row);
        let row_values = hash(&row_key);
        for col in 0..128 {
            if row_values[col] {
                grid[row][col] = counter;
                counter += 1;
            }
        }
    }

    // Connect regions
    //
    let mut regions = UnionFind::new(counter);
    for row in 0..128 {
        for col in 0..128 {
            let value = grid[row][col];
            if value != 0 {
                if row < 127 {
                    let value_right = grid[row + 1][col];
                    if value_right != 0 {
                        regions.join(value, value_right);
                    }
                }
                if col < 127 {
                    let value_down = grid[row][col + 1];
                    if value_down != 0 {
                        regions.join(value, value_down);
                    }
                }
            }
        }
    }

    println!("{}", regions.group_count() - 1);

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
