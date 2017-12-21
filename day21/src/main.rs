#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate failure;
use failure::Error;
extern crate strum;
#[macro_use]
extern crate strum_macros;

extern crate rayon;
use rayon::prelude::*;

use std::{io, process};
use std::io::BufRead;
use std::collections::HashMap;
use std::str::FromStr;
use std::str;
use std::char;
use std::ops::*;
use std::fmt;

extern crate aoc2017;
use aoc2017::matrix::*;

fn read_input() -> Result<(usize, Vec<(Matrix, Matrix)>), Error> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let iterations = lines.next().unwrap()?.parse()?;

    let mut rules = Vec::new();
    for line in lines {
        let line = line?;
        if !line.is_empty() {
            let mut parts = line.split(" => ");
            let source = parts.next().unwrap().parse()?;
            let target = parts.next().unwrap().parse()?;
            rules.push((source, target));
        }
    }

    Ok((iterations, rules))
}


fn print_rule(s: &Matrix, t: &Matrix) {
    println!("From: ");
    println!("{}", s);
    println!("To: ");
    println!("{}", t);

}

fn run() -> Result<(), Error> {
    let (iterations, rule_templates) = read_input()?;

    let mut rules: HashMap<Matrix, Matrix> = HashMap::with_capacity(8 * rule_templates.len());
    for (source, target) in rule_templates {
        let mut transformed_source = source;
        for _ in 0..4 {
            let next_transformed_source = transformed_source.rot90();
            rules.insert(transformed_source.flip(), target.clone());
            rules.insert(transformed_source, target.clone());
            transformed_source = next_transformed_source;
        }
    }

    let mut pattern: Matrix = ".#./..#/###".parse()?;

    for _ in 0..iterations {
        assert_eq!(pattern.height, pattern.width, "Must be square patterns");
        let source_wh = pattern.width;
        let source_stride = if source_wh % 2 == 0 { 2 } else { 3 };
        let steps = source_wh / source_stride;
        let target_stride = source_stride + 1;
        let target_wh = steps * target_stride;
        let mut next_pattern = Matrix::new(target_wh, target_wh);
        for square_x in 0..steps {
            for square_y in 0..steps {
                let source_square = pattern.slice(
                    square_x * source_stride,
                    square_y * source_stride,
                    source_stride,
                    source_stride,
                );
                if let Some(target_square) = rules.get(&source_square) {
                    next_pattern.fill_from(
                        square_x * target_stride,
                        square_y * target_stride,
                        target_square,
                    );
                } else {
                    bail!("Could not find rule for\n{}", source_square)
                }
            }
        }
        pattern = next_pattern;
    }

    //println!("{}", pattern);
    println!("{}", pattern.count_true());
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
