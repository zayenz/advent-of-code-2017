#![allow(dead_code)]

#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::io::BufRead;
use std::str::FromStr;
use std::fmt;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Program {
    identifier: u8,
}

impl Program {
    fn from_name(name: char) -> Program {
        Program { identifier: name as u8 }
    }
    fn from_ordinal(index: usize) -> Program {
        Program { identifier: (index + (b'a' as usize)) as u8 }
    }
    fn name(&self) -> char {
        self.identifier as char
    }
    fn index(&self) -> usize {
        (self.identifier - (b'a' as u8)) as usize
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}



#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Instruction {
    Spin { amount: usize },
    Exchange { first: usize, second: usize },
    Partner { first: Program, second: Program },
}

use Instruction::*;

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let instruction = s.chars().next().unwrap();
        Ok(if instruction == 's' {
            Spin { amount: s.get(1..).unwrap().parse()? }
        } else {
            let mut parts = s.get(1..).unwrap().split('/');
            let first = parts.next().unwrap();
            let second = parts.next().unwrap();
            match instruction {
                'x' => Exchange {
                    first: first.parse()?,
                    second: second.parse()?,
                },
                'p' => Partner {
                    first: Program::from_name(first.parse()?),
                    second: Program::from_name(second.parse()?),
                },
                _ => bail!("Could not parse \"{}\" as an instruction", s),
            }
        })
    }
}


fn read_input() -> Result<Vec<Instruction>, Error> {
    let mut input: Vec<Instruction> = Vec::new();
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



struct Programs {
    line: Vec<Program>,
}

impl Programs {
    fn new(max_program: usize) -> Programs {
        let mut line = Vec::new();
        for index in 0..max_program {
            line.push(Program::from_ordinal(index));
        }
        Programs { line }
    }

    fn position(&self, program: &Program) -> usize {
        self.line
            .iter()
            .enumerate()
            .find(|&(_, p)| p == program)
            .map(|(i, _)| i)
            .unwrap()
    }

    fn execute(&self, instruction: &Instruction) -> Programs {
        let new_line = match *instruction {
            Spin { amount } => {
                let length = self.line.len();
                self.line
                    .iter()
                    .cycle()
                    .skip(length - amount)
                    .take(length)
                    .cloned()
                    .collect()
            }
            Exchange { first, second } => {
                let mut new_line = self.line.clone();
                new_line.swap(first, second);
                new_line
            }
            Partner { first, second } => {
                let mut new_line = self.line.clone();
                let first_pos = self.position(&first);
                let second_pos = self.position(&second);
                new_line.swap(first_pos, second_pos);
                new_line

            }
        };
        Programs { line: new_line }
    }
}

impl fmt::Display for Programs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for program in &self.line {
            write!(f, "{}", program)?;
        }
        writeln!(f)
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "day16-1", about = "Solver for day 16, part 1.")]
struct Opt {
    /// An argument of type usize, with a default value.
    #[structopt(short = "p", long = "programs", help = "Number of programs", default_value = "16")]
    programs: usize,
}

fn run() -> Result<(), Error> {
    let instructions = read_input()?;
    let opt = Opt::from_args();

    let initial_programs = Programs::new(opt.programs);
    let final_programs = instructions.iter().fold(initial_programs, |programs,
     instruction| {
        programs.execute(instruction)
    });

    println!("{}", final_programs);

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
