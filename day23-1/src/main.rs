#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate failure;
use failure::{Error, Fail};

use std::{io, process};
use std::io::BufRead;
use std::str::FromStr;
use std::fmt;
use std::ops::{Index, IndexMut};

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
    last_sound: Option<ValueType>,
}

impl RegisterValue {
    fn new() -> RegisterValue {
        RegisterValue {
            value: BigInt::from(0),
            last_sound: None,
        }
    }

    fn play_sound(&mut self) {
        self.last_sound = Some(self.value.clone())
    }
}

impl fmt::Display for RegisterValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)?;
        if let Some(ref lsp) = self.last_sound {
            write!(f, "/{}", lsp)?;
        }
        Ok(())
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
    Snd { source: RegisterId },
    Set { target: RegisterId, source: Source },
    Add { target: RegisterId, source: Source },
    Sub { target: RegisterId, source: Source },
    Mul { target: RegisterId, source: Source },
    Mod { target: RegisterId, source: Source },
    Rcv { source: RegisterId },
    Jnz { condition: Source, offset: Source },
}

use Instruction::*;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Snd { ref source } => write!(f, "snd {}", source),
            Instruction::Set {
                ref target,
                ref source,
            } => write!(f, "set {} {}", target, source),
            Instruction::Add {
                ref target,
                ref source,
            } => write!(f, "add {} {}", target, source),
            Instruction::Sub {
                ref target,
                ref source,
            } => write!(f, "sub {} {}", target, source),
            Instruction::Mul {
                ref target,
                ref source,
            } => write!(f, "mul {} {}", target, source),
            Instruction::Mod {
                ref target,
                ref source,
            } => write!(f, "mod {} {}", target, source),
            Instruction::Rcv { ref source } => write!(f, "rcv {}", source),
            Instruction::Jnz {
                ref condition,
                ref offset,
            } => write!(f, "jnz {} {}", condition, offset),
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
            "sub" => Sub {
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
            "rcv" => Rcv { source: words.next().ok_or(MissingTokens)?.parse()? },
            "jnz" => Jnz {
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
    fn new() -> RegisterBank {
        let mut registers = Vec::with_capacity(REGISTERS);
        for _ in 0..REGISTERS {
            registers.push(RegisterValue::new())
        }
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



fn count_mul(instructions: &[Instruction]) -> Result<usize, Error> {
    let mut registers = RegisterBank::new();
    let mut mul_instructions = 0;
    let mut pc: isize = 0;
    let instruction_count = instructions.len() as isize;
    while 0 <= pc && pc < instruction_count {
        let instruction = &instructions[pc as usize];
        match *instruction {
            Instruction::Snd { ref source } => {
                registers[*source].play_sound();
                pc += 1;
            }
            Instruction::Set {
                ref target,
                ref source,
            } => {
                let value = source.value(&registers).clone();
                registers[*target].value = value;
                pc += 1;
            }
            Instruction::Add {
                ref target,
                ref source,
            } => {
                let target_value = &registers[*target].value.clone();
                let value = source.value(&registers).clone();
                registers[*target].value = target_value + value;
                pc += 1;
            }
            Instruction::Sub {
                ref target,
                ref source,
            } => {
                let target_value = &registers[*target].value.clone();
                let value = source.value(&registers).clone();
                registers[*target].value = target_value - value;
                pc += 1;
            }
            Instruction::Mul {
                ref target,
                ref source,
            } => {
                let target_value = &registers[*target].value.clone();
                let value = source.value(&registers).clone();
                registers[*target].value = target_value * value;
                pc += 1;
                mul_instructions += 1;
            }
            Instruction::Mod {
                ref target,
                ref source,
            } => {
                let target_value = &registers[*target].value.clone();
                let value = source.value(&registers).clone();
                registers[*target].value = target_value % value;
                pc += 1;
            }
            Instruction::Rcv { ref source } => {
                pc += 1;
            }
            Instruction::Jnz {
                ref condition,
                ref offset,
            } => {
                let condition = condition.value(&registers);
                if !condition.is_zero() {
                    pc = pc + offset.value(&registers).to_isize().unwrap();
                } else {
                    pc += 1;
                }
            }
        }
    }

    Ok(mul_instructions)
}

fn run() -> Result<(), Error> {
    let instructions = read_input()?;

    let mul_instruction = count_mul(&instructions)?;

    println!("{}", mul_instruction);

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
