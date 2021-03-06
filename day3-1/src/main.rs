extern crate failure;
use failure::Error;

use std::{io, process};
use std::cmp::{max, min};


#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

use Direction::*;

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match *self {
            Right => (1, 0),
            Up => (0, 1),
            Left => (-1, 0),
            Down => (0, -1),
        }
    }

    fn turn(&self) -> Direction {
        match *self {
            Right => Up,
            Up => Left,
            Left => Down,
            Down => Right,
        }
    }
}



#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct SpiralCell {
    index: i32,
    x: i32,
    y: i32,
    direction: Direction,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}

impl SpiralCell {
    fn new() -> SpiralCell {
        SpiralCell {
            index: 0,
            x: 0,
            y: 0,
            direction: Right,
            max_x: 0,
            max_y: 0,
            min_x: 0,
            min_y: 0,
        }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn next_location(&self) -> (i32, i32) {
        let (x_offset, y_offset) = self.direction.offset();
        (self.x + x_offset, self.y + y_offset)
    }

    fn is_inside(&self, x: i32, y: i32) -> bool {
        self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y
    }

    fn next(&mut self) -> SpiralCell {
        let (next_x, next_y) = self.next_location();

        let next_direction = if self.is_inside(next_x, next_y) {
            self.direction
        } else {
            self.direction.turn()
        };

        SpiralCell {
            index: self.index + 1,
            x: next_x,
            y: next_y,
            direction: next_direction,
            max_x: max(self.max_x, next_x),
            max_y: max(self.max_y, next_y),
            min_x: min(self.min_x, next_x),
            min_y: min(self.min_y, next_y),
        }
    }
}



fn read_input() -> Result<i32, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.parse()?)
}


fn run() -> Result<(), Error> {
    let input = read_input()?;

    let mut cell = SpiralCell::new();
    for _ in 0..input - 1 {
        cell = cell.next();
    }

    let distance = cell.distance();

    println!("{}", distance);

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
