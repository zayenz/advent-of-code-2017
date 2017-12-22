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

const STEPS: usize = 10_000_000;

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
    Same,
    Back,
}

use Turn::*;

impl Direction {
    fn back(&self) -> Direction {
        match *self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }

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
            (_, Same) => *self,
            (_, Back) => self.back(),
        }
    }
}

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

use State::*;

impl State {
    fn step(&self) -> State {
        match *self {
            Clean => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged => Clean,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Grid {
    states: HashMap<Position, State>,
    min_x: Scalar,
    min_y: Scalar,
    max_x: Scalar,
    max_y: Scalar,
}

impl Grid {
    fn new() -> Grid {
        let mut states = HashMap::with_capacity(STEPS);
        states.insert(Position::new(0, 0), Clean);
        Grid {
            states,
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

    fn set(&mut self, position: Position, state: State) {
        self.update_bounds(position);
        self.states.insert(position, state);
    }

    fn get(&mut self, position: Position) -> State {
        self.update_bounds(position);
        *self.states.get(&position).unwrap_or(&Clean)
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
            let position = Position::new(col as Scalar, row as Scalar);
            if ch == '#' {
                grid.set(position, Infected);
            } else {
                grid.set(position, Clean);
            }
            max_col = max(max_col, col);
        }
        max_row = max(max_row, row);
    }

    let center = Position::new(max_col as Scalar / 2, max_row as Scalar / 2);

    Ok((center, grid))
}



fn run() -> Result<(), Error> {
    let (center, start_grid) = read_input()?;

    let mut infection_count = 0;
    let mut grid = start_grid.clone();
    let mut position = center;
    let mut direction = North;
    for _ in 0..STEPS {
        let turn = match grid.get(position) {
            Clean => Left,
            Weakened => Same,
            Infected => Right,
            Flagged => Back,
        };
        let next_state = grid.get(position).step();
        if next_state == Infected {
            infection_count += 1;
        }
        grid.set(position, next_state);
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
