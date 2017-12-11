#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    North,
    NorthWest,
    NorthEast,
    SouthEast,
    SouthWest,
    South,
}

use Direction::*;

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "n" => North,
            "nw" => NorthWest,
            "ne" => NorthEast,
            "sw" => SouthWest,
            "se" => SouthEast,
            "s" => South,
            _ => bail!("Could not parse \"{}\" as a direection", s),
        })
    }
}


fn read_input() -> Result<Vec<Direction>, Error> {
    let mut input: Vec<Direction> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for word in line?.trim().split(',') {
            if !word.is_empty() {
                input.push(word.parse()?);
            }
        }
    }
    Ok(input)
}



/// See https://www.redblobgames.com/grids/hexagons/#coordinates-cube for a discussion
/// on how the hax coordinates work.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct HexCoordinate {
    x: i32,
    y: i32,
    z: i32,
}


impl HexCoordinate {
    fn origo() -> HexCoordinate {
        HexCoordinate::default()
    }

    /// See https://www.redblobgames.com/grids/hexagons/#distances-cube for motivation
    /// Equivalently, one could take max of the three components; one of the three parts will
    /// be the sum of the other two by construction.
    fn distance(&self, other: HexCoordinate) -> i32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }

    fn distance_to_origo(&self) -> i32 {
        self.distance(HexCoordinate::origo())
    }

    fn step(&self, direction: Direction) -> HexCoordinate {
        match direction {
            Direction::North => HexCoordinate {
                x: self.x,
                y: self.y + 1,
                z: self.z - 1,
            },
            Direction::NorthWest => HexCoordinate {
                x: self.x + 1,
                y: self.y,
                z: self.z - 1,
            },
            Direction::NorthEast => HexCoordinate {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z,
            },
            Direction::SouthEast => HexCoordinate {
                x: self.x - 1,
                y: self.y,
                z: self.z + 1,
            },
            Direction::SouthWest => HexCoordinate {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z,
            },
            Direction::South => HexCoordinate {
                x: self.x,
                y: self.y - 1,
                z: self.z + 1,
            },
        }
    }
}


fn run() -> Result<(), Error> {
    let directions = read_input()?;

    let mut position = HexCoordinate::origo();

    for direction in directions {
        position = position.step(direction);
    }

    println!("{}", position.distance_to_origo());

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
