#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Node {
    name: String,
    weight: i32,
}

impl Node {
    fn new(name: String, weight: i32) -> Node {
        Node { name, weight }
    }
}

fn read_input() -> Result<HashMap<Node, HashSet<Node>>, Error> {
    let mut input: HashMap<Node, HashSet<String>> = HashMap::new();
    let mut node_by_name = HashMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut words = line.split_whitespace();
        if let Some(node_name) = words.next() {
            let weight: i32 = words
                .next()
                .unwrap()
                .trim_matches('(')
                .trim_matches(')')
                .parse()?;
            let mut children = HashSet::new();
            if let Some(_arrow) = words.next() {
                for child in words {
                    children.insert(child.trim_matches(',').to_string());
                }
            }
            let node = Node::new(node_name.to_string(), weight);
            node_by_name.insert(node.name.clone(), node.clone());
            input.insert(node, children);
        }
    }

    let mut result = HashMap::new();
    for (head, child_names) in input {
        let mut children = HashSet::new();
        for child_name in child_names {
            children.insert(node_by_name.get(&child_name).unwrap().clone());
        }
        result.insert(head, children);
    }

    Ok(result)
}


fn check_recursive(root: &Node, tree: &HashMap<Node, HashSet<Node>>) -> i32 {
    let mut child_weights: HashMap<Node, i32> = HashMap::new();
    let mut weights: Vec<i32> = Vec::new();
    for child in tree.get(root).unwrap() {
        let weight = check_recursive(child, tree);
        child_weights.insert(child.clone(), weight);
        weights.push(weight);
    }

    let mut adjustment: i32 = 0;
    if weights.len() >= 3 {
        let mut weight_count: HashMap<i32, i32> = HashMap::new();
        for weight in &weights {
            let count = weight_count.entry(*weight).or_insert(0);
            *count += 1;
        }
        if weight_count.len() > 1 {
            let (correct, _) = weight_count.iter().max_by_key(|&(_k, v)| v).unwrap();
            let (wrong, _) = weight_count.iter().min_by_key(|&(_k, v)| v).unwrap();
            let (unbalanced_child, _) = child_weights
                .iter()
                .find(|&(_, weight)| weight == wrong)
                .unwrap();
            adjustment = correct - wrong;
            println!("{}", unbalanced_child.weight + adjustment);
        }
    }

    let all_child_weights: i32 = weights.iter().sum();
    return root.weight + all_child_weights + adjustment;
}

fn run() -> Result<(), Error> {
    let tree = read_input()?;

    let mut has_parent = HashSet::new();

    for children in tree.values() {
        for child in children {
            has_parent.insert(child.clone());
        }
    }

    for node in tree.keys() {
        if !has_parent.contains(node) {
            check_recursive(&node, &tree);
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
