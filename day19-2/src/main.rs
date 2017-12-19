#[macro_use]
extern crate failure;
use failure::Error;
extern crate strum;
#[macro_use]
extern crate strum_macros;


use std::{io, process};
use std::io::BufRead;
use std::str::FromStr;
use std::str;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Cell {
    Empty,
    Corner,
    Letter(char),
    Path,
}

use Cell::*;

impl Cell {}

impl FromStr for Cell {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(
            s.len() == 1,
            "Wrong length of string, expected 1 but got {}",
            s.len()
        );
        Ok(match s {
            " " => Empty,
            "+" => Corner,
            "|" | "-" => Path,
            _ => {
                let ch = s.chars().next().unwrap();
                ensure!(
                    ch.is_alphabetic(),
                    "Unrecognized letter. Expected [A-Za-z] but got {}",
                    ch
                );
                Letter(ch)
            }
        })
    }
}


fn read_input() -> Result<Vec<Vec<Cell>>, Error> {
    let mut rows = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if !line.is_empty() {
            let mut row = Vec::new();
            row.push(Empty);
            for cell in line.chars() {
                row.push(cell.to_string().parse()?);
            }
            row.push(Empty);
            rows.push(row);
        }
    }
    let width = rows.iter().map(|r| r.len()).max().unwrap();
    let mut empty_row = Vec::with_capacity(width);
    for _ in 0..width {
        empty_row.push(Empty);
    }
    rows.insert(0, empty_row.clone());
    rows.push(empty_row);

    for row in &mut rows {
        while row.len() < width {
            row.push(Empty);
        }
    }

    Ok(rows)
}


#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

use Direction::*;

impl Direction {
    fn lr(&self) -> [Direction; 2] {
        match *self {
            Up | Down => [Right, Left],
            Right | Left => [Up, Down],
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn position(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    fn step(&self, direction: Direction) -> Position {
        let (x, y) = match direction {
            Up => (self.x - 1, self.y),
            Down => (self.x + 1, self.y),
            Left => (self.x, self.y - 1),
            Right => (self.x, self.y + 1),
        };
        Position { x, y }
    }
}


fn get(grid: &[Vec<Cell>], pos: Position) -> Cell {
    grid[pos.x][pos.y]
}

fn run() -> Result<(), Error> {
    let grid = read_input()?;
    let entrance_column = grid[1]
        .iter()
        .enumerate()
        .find(|&(_, &cell)| cell != Empty)
        .map(|(index, _)| index)
        .unwrap();
    let mut pos = Position::position(1, entrance_column);
    let mut dir = Down;
    let mut steps = 0;
    loop {
        let (next_pos, next_dir) = match get(&grid, pos) {
            Letter(_) | Path => (pos.step(dir), dir),
            Corner => {
                let next_direction = *dir.lr()
                    .iter()
                    .find(|&&d| get(&grid, pos.step(d)) != Empty)
                    .unwrap();

                (pos.step(next_direction), next_direction)
            }
            Empty => {
                break;
            }
        };
        steps += 1;
        pos = next_pos;
        dir = next_dir;
    }

    println!("{}", steps);

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
