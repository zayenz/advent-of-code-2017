#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;


fn read_input() -> Result<HashMap<String, HashSet<String>>, Error> {
    let mut input: HashMap<String, HashSet<String>> = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut words = line.split_whitespace();
        if let Some(node) = words.next() {
            let _weight = words.next().unwrap();
            let mut children = HashSet::new();
            if let Some(_arrow) = words.next() {
                for child in words {
                    children.insert(child.trim_matches(',').to_string());
                }
            }
            input.insert(node.to_string(), children);
        }
    }
    Ok(input)
}



fn run() -> Result<(), Error> {
    let tree = read_input()?;

    let mut has_parent = HashSet::new();

    for children in tree.values() {
        for child in children.iter() {
            has_parent.insert(child.clone());
        }
    }

    for node in tree.keys() {
        if !has_parent.contains(node) {
            println!("{}", node);
            return Ok(());
        }
    }

    bail!("No root found")
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
