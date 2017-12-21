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


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Matrix {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Matrix {
    fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            width,
            height,
            data: vec![false; width * height],
        }
    }

    fn count_true(&self) -> usize {
        self.data.iter().filter(|&&v| v).count()
    }


    fn pos(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y < self.height);
        x + (y * self.width)
    }

    fn slice(&self, start_x: usize, start_y: usize, width: usize, height: usize) -> Matrix {
        let mut data = Vec::with_capacity(width * height);
        for x in start_x..(start_x + width) {
            for y in start_y..(start_y + height) {
                data.push(self[(x, y)])
            }
        }
        Matrix {
            width,
            height,
            data,
        }
    }

    fn fill_from(&mut self, x: usize, y: usize, source: &Matrix) {
        for source_x in 0..source.width {
            for source_y in 0..source.height {
                self[(x + source_x, y + source_y)] = source[(source_x, source_y)];
            }
        }
    }

    fn rot90(&self) -> Matrix {
        let mut result = Matrix::new(self.height, self.width);

        for x in 0..self.width {
            for y in 0..self.height {
                let xr90 = y;
                let yr90 = self.height - x - 1;
                result[(xr90, yr90)] = self[(x, y)];
            }
        }

        result
    }

    fn flip(&self) -> Matrix {
        let mut result = Matrix::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                result[(self.width - x - 1, y)] = self[(x, y)];
            }
        }

        result
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.data[self.pos(x, y)]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        let position = self.pos(x, y);
        &mut self.data[position]
    }
}

impl FromStr for Matrix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let rows: Vec<&str> = s.split('/').collect();
        if rows.is_empty() {
            return Ok(Matrix::new(0, 0));
        }
        let width = rows[0].len();
        let height = rows.len();
        let mut result = Matrix::new(width, height);
        for (y, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                result[(x, y)] = ch == '#';
            }
        }
        Ok(result)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self[(x, y)] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

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
