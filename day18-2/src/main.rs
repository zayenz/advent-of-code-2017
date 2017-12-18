#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate failure;
use failure::{Error, Fail};

use std::{io, process};
use std::io::BufRead;
use std::str::FromStr;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::collections::VecDeque;

extern crate num;
use num::{FromPrimitive, ToPrimitive, Zero};
use num::bigint::BigInt;

#[derive(Fail, Debug)]
#[fail(display = "Not a valid name: {}.", _0)]
struct NotValidName(char);

#[derive(Fail, Debug)]
#[fail(display = "Not a valid index for register: {}.", _0)]
struct NotValidIndex(usize);

#[derive(Fail, Debug)]
#[fail(display = "Name is too long: {}.", _0)]
struct TooLongName(String);

#[derive(Fail, Debug)]
#[fail(display = "Not enough tokens to parse")]
struct MissingTokens;

#[derive(Fail, Debug)]
#[fail(display = "Value is too large")]
struct TooLargeValue;


type ValueType = BigInt;

const REGISTERS: usize = 26;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RegisterId {
    identifier: u8,
}

impl RegisterId {
    fn from_name(name: char) -> Result<RegisterId, Error> {
        let id = ((name as usize) - b'a' as usize) as u8;
        if id <= (REGISTERS as u8) {
            Ok(RegisterId { identifier: id })
        } else {
            Err(Error::from(NotValidName(name)))
        }
    }

    fn from_index(index: usize) -> Result<RegisterId, Error> {
        if index <= REGISTERS {
            Ok(RegisterId { identifier: index as u8 })
        } else {
            Err(Error::from(NotValidIndex(index)))
        }
    }

    fn index(&self) -> usize {
        self.identifier as usize
    }

    fn name(&self) -> char {
        ((self.identifier + b'a') as char)
    }
}

impl fmt::Display for RegisterId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for RegisterId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.trim();
        if name.len() > 1 {
            Err(Error::from(TooLongName(name.to_string())))
        } else {
            Ok(RegisterId::from_name(name.chars().next().unwrap())?)
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RegisterValue {
    value: ValueType,
}

impl RegisterValue {
    fn new() -> RegisterValue {
        RegisterValue { value: BigInt::from(0) }
    }
}

impl fmt::Display for RegisterValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Source {
    Register { id: RegisterId },
    Value { value: ValueType },
}

use Source::*;

impl Source {
    fn value<'a>(&'a self, registers: &'a RegisterBank) -> &'a ValueType {
        match *self {
            Source::Register { id } => &registers[id].value,
            Source::Value { ref value } => value,
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Source::Register { id } => write!(f, "reg({})", id),
            Source::Value { ref value } => write!(f, "val({})", value),
        }

    }
}

impl FromStr for Source {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.trim();
        Ok(if let Ok(value) = name.parse() {
            Value { value }
        } else {
            Register { id: name.parse()? }
        })
    }
}


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Instruction {
    Snd { source: Source },
    Set { target: RegisterId, source: Source },
    Add { target: RegisterId, source: Source },
    Mul { target: RegisterId, source: Source },
    Mod { target: RegisterId, source: Source },
    Rcv { target: RegisterId },
    Jgz { condition: Source, offset: Source },
}

use Instruction::*;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Snd { ref source } => write!(f, "snd {}", source),
            Set {
                ref target,
                ref source,
            } => write!(f, "set {} {}", target, source),
            Add {
                ref target,
                ref source,
            } => write!(f, "add {} {}", target, source),
            Mul {
                ref target,
                ref source,
            } => write!(f, "mul {} {}", target, source),
            Mod {
                ref target,
                ref source,
            } => write!(f, "mod {} {}", target, source),
            Rcv { ref target } => write!(f, "rcv {}", target),
            Jgz {
                ref condition,
                ref offset,
            } => write!(f, "jgz {} {}", condition, offset),
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let instruction = words.next().ok_or(MissingTokens)?;
        Ok(match instruction {
            "snd" => Snd { source: words.next().ok_or(MissingTokens)?.parse()? },
            "set" => Set {
                target: words.next().ok_or(MissingTokens)?.parse()?,
                source: words.next().ok_or(MissingTokens)?.parse()?,
            },
            "add" => Add {
                target: words.next().ok_or(MissingTokens)?.parse()?,
                source: words.next().ok_or(MissingTokens)?.parse()?,
            },
            "mod" => Mod {
                target: words.next().ok_or(MissingTokens)?.parse()?,
                source: words.next().ok_or(MissingTokens)?.parse()?,
            },
            "mul" => Mul {
                target: words.next().ok_or(MissingTokens)?.parse()?,
                source: words.next().ok_or(MissingTokens)?.parse()?,
            },
            "rcv" => Rcv { target: words.next().ok_or(MissingTokens)?.parse()? },
            "jgz" => Jgz {
                condition: words.next().ok_or(MissingTokens)?.parse()?,
                offset: words.next().ok_or(MissingTokens)?.parse()?,
            },
            _ => bail!("Could not parse \"{}\" as an instruction", s),
        })
    }
}


fn read_input() -> Result<Vec<Instruction>, Error> {
    let mut input: Vec<Instruction> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if !line.is_empty() {
            input.push(line.parse()?);
        }
    }
    Ok(input)
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RegisterBank {
    registers: Vec<RegisterValue>,
}

impl RegisterBank {
    fn new(id: usize) -> RegisterBank {
        let mut registers = Vec::with_capacity(REGISTERS);
        for _ in 0..REGISTERS {
            registers.push(RegisterValue::new())
        }
        registers[RegisterId::from_name('p').unwrap().index()].value = BigInt::from(id);
        RegisterBank { registers }
    }
}

impl Index<RegisterId> for RegisterBank {
    type Output = RegisterValue;

    fn index(&self, id: RegisterId) -> &RegisterValue {
        &self.registers[id.index()]
    }
}

impl IndexMut<RegisterId> for RegisterBank {
    fn index_mut(&mut self, id: RegisterId) -> &mut RegisterValue {
        &mut self.registers[id.index()]
    }
}

impl fmt::Display for RegisterBank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for index in 0..REGISTERS {
            write!(
                f,
                "{}:{},",
                RegisterId::from_index(index).unwrap(),
                self.registers[index]
            )?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Status {
    Waiting,
    Send { source: usize, value: ValueType },
    SingleStep,
}

use Status::*;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Program {
    id: usize,
    pc: usize,
    instructions: Vec<Instruction>,
    registers: RegisterBank,
    input: VecDeque<ValueType>,
    sent_value: usize,
}

impl Program {
    fn new(id: usize, instructions: &[Instruction]) -> Program {
        Program {
            id,
            pc: 0,
            instructions: Vec::from(instructions),
            registers: RegisterBank::new(id),
            input: VecDeque::new(),
            sent_value: 0,
        }
    }

    fn input(&mut self, input: ValueType) {
        self.input.push_back(input);
    }

    fn step(&mut self) -> Result<Status, Error> {
        assert!(self.pc < self.instructions.len());
        let instruction = &self.instructions[self.pc];
        Ok(match *instruction {
            Snd { ref source } => {
                self.pc += 1;
                self.sent_value += 1;
                let value = source.value(&self.registers).clone();
                Send {
                    source: self.id,
                    value,
                }
            }
            Set {
                ref target,
                ref source,
            } => {
                self.pc += 1;
                let value = source.value(&self.registers).clone();
                self.registers[*target].value = value;
                SingleStep
            }
            Add {
                ref target,
                ref source,
            } => {
                self.pc += 1;
                let target_value = &self.registers[*target].value.clone();
                let value = source.value(&self.registers).clone();
                self.registers[*target].value = target_value + value;
                SingleStep
            }
            Mul {
                ref target,
                ref source,
            } => {
                self.pc += 1;
                let target_value = &self.registers[*target].value.clone();
                let value = source.value(&self.registers).clone();
                self.registers[*target].value = target_value * value;
                SingleStep
            }
            Mod {
                ref target,
                ref source,
            } => {
                self.pc += 1;
                let target_value = &self.registers[*target].value.clone();
                let value = source.value(&self.registers).clone();
                self.registers[*target].value = target_value % value;
                SingleStep
            }
            Rcv { ref target } => {
                if let Some(input) = self.input.pop_front() {
                    self.pc += 1;
                    self.registers[*target].value = input;
                    SingleStep
                } else {
                    Waiting
                }
            }
            Jgz {
                ref condition,
                ref offset,
            } => {
                let condition = condition.value(&self.registers);
                if *condition > BigInt::zero() {
                    let offset = offset.value(&self.registers).to_isize().ok_or(
                        TooLargeValue,
                    )?;
                    self.pc = (self.pc as isize + offset) as usize;
                } else {
                    self.pc += 1;
                }
                SingleStep
            }
        })
    }
}



fn simulate(instructions: &[Instruction]) -> Result<Vec<Program>, Error> {
    let mut programs = vec![
        Program::new(0, instructions),
        Program::new(1, instructions),
    ];
    loop {
        let results = programs
            .iter_mut()
            .map(Program::step)
            .collect::<Result<Vec<_>, _>>()?;
        if results.iter().all(|s| *s == Waiting) {
            return Ok(programs);
        }
        for result in results {
            if let Send { source, ref value } = result {
                for program in &mut programs {
                    if program.id != source {
                        program.input(value.clone())
                    }
                }
            }
        }
    }
}

fn run() -> Result<(), Error> {
    let instructions = read_input()?;

    let programs = simulate(&instructions)?;
    let sends_from_1 = programs.iter().find(|p| p.id == 1).unwrap().sent_value;

    println!("{}", sends_from_1);

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
