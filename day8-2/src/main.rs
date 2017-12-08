#[macro_use]
extern crate failure;
use failure::Error;

use std::{io, process};
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Comparison {
    Less,
    LessEqual,
    Equal,
    GreaterEqual,
    Greater,
    NotEqual,
}

use Comparison::*;

impl Comparison {
    fn compare(&self, lhs: i32, rhs: i32) -> bool {
        match *self {
            Less => lhs < rhs,
            LessEqual => lhs <= rhs,
            Equal => lhs == rhs,
            GreaterEqual => lhs >= rhs,
            Greater => lhs > rhs,
            NotEqual => lhs != rhs,
        }
    }
}

impl FromStr for Comparison {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "<" => Less,
            "<=" => LessEqual,
            "==" => Equal,
            ">=" => GreaterEqual,
            ">" => Greater,
            "!=" => NotEqual,
            _ => bail!("Could not parse \"{}\" as a comparison", s),
        })
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Instruction {
    mod_reg: String,
    mod_val: i32,
    cond_reg: String,
    cond: Comparison,
    cond_val: i32,
}

impl Instruction {
    fn new(
        mod_reg: String,
        mod_val: i32,
        cond_reg: String,
        cond: Comparison,
        cond_val: i32,
    ) -> Instruction {
        Instruction {
            mod_reg,
            mod_val,
            cond_reg,
            cond,
            cond_val,
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut words = line.split_whitespace();
        let mod_reg = words.next().unwrap().to_string();
        let mod_val = {
            let sign: i32 = {
                match words.next().unwrap() {
                    "inc" => 1,
                    "dec" => -1,
                    direction => bail!("Unknown direction \"{}\"", direction),
                }
            };
            let mod_val_base: i32 = words.next().unwrap().parse()?;
            sign * mod_val_base
        };
        words.next(); // "if"
        let cond_reg = words.next().unwrap().to_string();
        let cond = words.next().unwrap().parse()?;
        let cond_val = words.next().unwrap().parse()?;

        Ok(Instruction::new(mod_reg, mod_val, cond_reg, cond, cond_val))
    }
}


fn read_input() -> Result<Vec<Instruction>, Error> {
    let mut input = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            input.push(line.parse()?)
        }
    }
    Ok(input)
}



fn run() -> Result<(), Error> {
    let instructions = read_input()?;

    let mut registers: HashMap<String, i32> = HashMap::new();

    let mut max_register = 0;

    for instruction in instructions {
        let cond_reg = *registers.get(&instruction.cond_reg).unwrap_or(&0);
        if instruction.cond.compare(cond_reg, instruction.cond_val) {
            let reg_val = registers.entry(instruction.mod_reg).or_insert(0);
            *reg_val += instruction.mod_val;
            if *reg_val > max_register {
                max_register = *reg_val;
            }
        }
    }

    println!("{}", max_register);

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
