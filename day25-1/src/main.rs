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
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::str;
use std::char;
use std::ops::*;
use std::fmt;

extern crate aoc2017;
use aoc2017::matrix::*;

#[derive(EnumString, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Direction {
    #[strum(serialize = "right")]
    Right,
    #[strum(serialize = "left")]
    Left,
}

use Direction::*;

type Symbol = u8;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Tape {
    data: VecDeque<Symbol>,
    pos: usize,
}

impl Tape {
    fn new() -> Tape {
        let mut data = VecDeque::with_capacity(10);
        data.push_front(0);
        Tape { data, pos: 0 }
    }

    fn read(&self) -> Symbol {
        self.data[self.pos]
    }

    fn checksum(&self) -> usize {
        self.data.iter().filter(|&&v| v == 1).count()
    }

    fn step(&mut self, direction: Direction) {
        match direction {
            Right => {
                if self.pos == self.data.len() - 1 {
                    self.data.push_back(0);
                }
                self.pos += 1;
            }
            Left => {
                if self.pos == 0 {
                    self.data.push_front(0);
                } else {
                    self.pos -= 1;
                }
            }
        }
    }

    fn write(&mut self, value: Symbol) {
        self.data[self.pos] = value;
    }
}

type StateName = char;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Action {
    write: Symbol,
    step: Direction,
    next: StateName,
}

#[derive(Debug, Clone)]
struct Machine {
    tape: Tape,
    state: StateName,
    states: HashMap<StateName, HashMap<Symbol, Action>>,
}

impl Machine {
    fn step(&mut self) {
        let current_value: Symbol = self.tape.read();
        let action: Action = self.states[&self.state][&current_value];
        self.tape.write(action.write);
        self.tape.step(action.step);
        self.state = action.next;
    }
}


fn is_extra_word(word: &str) -> bool {
    match word.to_lowercase().as_ref() {
        "" | "-" | "begin" | "in" | "state" | "perform" | "diagnostic" | "checksum" | "after" |
        "steps" | "if" | "the" | "current" | "value" | "is" | "write" | "move" | "one" |
        "slot" | "to" | "continue" | "with" => true,
        _ => false,
    }
}

fn read_input() -> Result<(usize, Machine), Error> {
    let stdin = io::stdin();
    let mut words = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim().trim_matches('.').trim_matches(':');
        for word in line.split_whitespace().filter(|w| !is_extra_word(w)) {
            words.push(word.to_string())
        }
    }
    let initial_state: StateName = words[0].parse()?;
    let iterations: usize = words[2].parse()?;
    let mut states = HashMap::new();
    for state in words[3..].chunks(9) {
        let name: StateName = state[0].parse()?;
        let mut transitions = HashMap::new();
        for transition_data in state[1..].chunks(4) {
            let condition: Symbol = transition_data[0].parse()?;
            let state = Action {
                write: transition_data[1].parse()?,
                step: transition_data[2].parse()?,
                next: transition_data[3].parse()?,
            };
            transitions.insert(condition, state);
        }
        states.insert(name, transitions);
    }

    let machine = Machine {
        tape: Tape::new(),
        state: initial_state,
        states,
    };
    Ok((iterations, machine))
}


fn run() -> Result<(), Error> {
    let (iterations, mut machine) = read_input()?;

    for _ in 0..iterations {
        machine.step();
    }

    println!("{}", machine.tape.checksum());
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
