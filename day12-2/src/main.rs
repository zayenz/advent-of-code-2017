#![allow(dead_code)]

extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;
use std::cmp::max;



fn read_input() -> Result<(i32, Vec<Vec<i32>>), Error> {
    let mut largest = 0;
    let mut input: Vec<Vec<i32>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut words = line.trim().split_whitespace();
        if let Some(first_word) = words.next() {
            let mut group = Vec::new();
            group.push(first_word.parse()?);
            words.next(); // <->
            for word in words {
                group.push(word.trim_right_matches(',').parse()?)
            }
            largest = max(largest, *group.iter().max().unwrap());
            input.push(group);
        }
    }
    Ok((largest, input))
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


fn run() -> Result<(), Error> {
    let (largest, input_groups) = read_input()?;

    let mut groups = UnionFind::new(largest + 1);

    for input_group in &input_groups {
        for a in input_group {
            for b in input_group {
                groups.join(*a, *b);
            }
        }
    }

    println!("{}", groups.group_count());

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
