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
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::str;
use std::char;
use std::ops::*;
use std::fmt;
use std::cmp::{min, max};

const STEPS: usize = 10_000;

type Scalar = i16;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: Scalar,
    y: Scalar,
}

impl Position {
    fn new(x: Scalar, y: Scalar) -> Position {
        Position { x, y }
    }

    fn step(&self, direction: Direction) -> Position {
        let (x, y) = match direction {
            North => (self.x, self.y - 1),
            South => (self.x, self.y + 1),
            West => (self.x - 1, self.y),
            East => (self.x + 1, self.y),
        };
        Position { x, y }
    }
}

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::*;

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Turn {
    Left,
    Right,
}

use Turn::*;

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match (*self, turn) {
            (North, Left) => West,
            (North, Right) => East,
            (South, Left) => East,
            (South, Right) => West,
            (East, Left) => North,
            (East, Right) => South,
            (West, Left) => South,
            (West, Right) => North,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Grid {
    infected: HashSet<Position>,
    min_x: Scalar,
    min_y: Scalar,
    max_x: Scalar,
    max_y: Scalar,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            infected: HashSet::with_capacity(STEPS),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    fn update_bounds(&mut self, position: Position) {
        self.min_x = min(self.min_x, position.x);
        self.min_y = min(self.min_y, position.y);
        self.max_x = max(self.max_x, position.x);
        self.max_y = max(self.max_y, position.y);
    }

    fn infect(&mut self, position: Position) {
        self.update_bounds(position);
        self.infected.insert(position);
    }

    /// Toggle the position (and update the bounds).
    /// Returns if the node was infected before toggling.
    fn toggle(&mut self, position: Position) -> bool {
        self.update_bounds(position);
        if self.infected.contains(&position) {
            self.infected.remove(&position);
            true
        } else {
            self.infected.insert(position);
            false
        }
    }

    fn count_infected(&self) -> usize {
        self.infected.len()
    }
}

fn read_input() -> Result<(Position, Grid), Error> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut max_col = 0;
    let mut max_row = 0;
    let mut grid = Grid::new();
    for (row, line) in lines.enumerate() {
        let line = line?;
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                grid.infect(Position::new(col as Scalar, row as Scalar));
            }
            max_col = max(max_col, col);
        }
        max_row = max(max_row, row);
    }

    let center = Position::new(max_col as Scalar / 2, max_row as Scalar / 2);

    Ok((center, grid))
}


fn print(grid: &Grid, position: Position) {
    for y in grid.min_y..(grid.max_y + 1) {
        for x in grid.min_x..(grid.max_x + 1) {
            let current = Position::new(x, y);
            let cell = if grid.infected.contains(&current) {
                '#'
            } else {
                '.'
            };
            if current == position {
                print!("[{}]", cell);
            } else {
                print!(" {} ", cell);
            }
        }
        println!();
    }
}

fn run() -> Result<(), Error> {
    let (center, start_grid) = read_input()?;

    let mut infection_count = 0;
    let mut grid = start_grid.clone();
    let mut position = center;
    let mut direction = North;
    for _ in 0..STEPS {
        //        println!("--------");
        //        print(&grid, position);
        //        println!("--------");
        let was_infected = grid.toggle(position);
        if !was_infected {
            infection_count += 1;
        }
        let turn = if was_infected { Right } else { Left };
        direction = direction.turn(turn);
        position = position.step(direction);
    }

    println!("{}", infection_count);
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
