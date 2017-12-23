#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate failure;
use failure::{Error, Fail};

use std::{io, process};
use std::io::BufRead;
use std::str::FromStr;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::collections::HashMap;

extern crate num;
use num::{FromPrimitive, ToPrimitive, Zero, One};
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


type ValueType = isize;

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
            value: 0,
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
enum JumpTarget {
    Offset { offset: isize },
    Symbolic { label: String },
}

use JumpTarget::*;

impl fmt::Display for JumpTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Offset { offset } => write!(f, "{}", offset),
            Symbolic { ref label } => write!(f, "symbolic({})", label),
        }

    }
}

impl FromStr for JumpTarget {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.trim();
        Ok(if name.starts_with('\'') {
            Symbolic { label: name.trim_matches('\'').to_string() }
        } else {
            Offset { offset: name.parse()? }
        })
    }
}


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Instruction {
    Set { target: RegisterId, source: Source },
    Add { target: RegisterId, source: Source },
    Sub { target: RegisterId, source: Source },
    Mul { target: RegisterId, source: Source },
    Mod { target: RegisterId, source: Source },
    Jnq {
        condition1: Source,
        condition2: Source,
        offset: JumpTarget,
    },
    Jnz {
        condition: Source,
        offset: JumpTarget,
    },
}

use Instruction::*;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
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
            Instruction::Jnq {
                ref condition1,
                ref condition2,
                ref offset,
            } => write!(f, "jnq {} {} {}", condition1, condition2, offset),
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
            "jnq" => Jnq {
                condition1: words.next().ok_or(MissingTokens)?.parse()?,
                condition2: words.next().ok_or(MissingTokens)?.parse()?,
                offset: words.next().ok_or(MissingTokens)?.parse()?,
            },
            "jnz" => Jnz {
                condition: words.next().ok_or(MissingTokens)?.parse()?,
                offset: words.next().ok_or(MissingTokens)?.parse()?,
            },
            _ => bail!("Could not parse \"{}\" as an instruction", s),
        })
    }
}


fn read_input() -> Result<Vec<Instruction>, Error> {
    let mut symbolic_input: Vec<Instruction> = Vec::new();
    let stdin = io::stdin();
    let mut line_count: isize = 0;
    let mut name_list: HashMap<String, isize> = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line?;
        if !line.is_empty() {
            let mut parts = line.split(": ");
            let first_part = parts.next().unwrap();
            let instruction_string = if let Some(instruction) = parts.next() {
                name_list.insert(first_part.to_string(), line_count);
                instruction
            } else {
                first_part
            };
            let instruction: Instruction = instruction_string.parse()?;
            symbolic_input.push(instruction);
            line_count += 1;
        }
    }

    //    print_instructions(&symbolic_input);
    let mut instructions: Vec<Instruction> = Vec::new();

    for (current_line, instruction) in symbolic_input.iter().enumerate() {
        match *instruction {
            Instruction::Jnq {
                ref condition1,
                ref condition2,
                ref offset,
            } => {
                if let Symbolic { label } = offset.clone() {
                    let target_line = name_list[&label];
                    instructions.push(Jnq {
                        condition1: condition1.clone(),
                        condition2: condition2.clone(),
                        offset: Offset { offset: target_line - (current_line as isize) },
                    });
                } else {
                    instructions.push(instruction.clone());
                }
            }
            Instruction::Jnz {
                ref condition,
                ref offset,
            } => {
                if let Symbolic { label } = offset.clone() {
                    let target_line = name_list[&label];
                    instructions.push(Jnz {
                        condition: condition.clone(),
                        offset: Offset { offset: target_line - (current_line as isize) },
                    });
                } else {
                    instructions.push(instruction.clone());
                }
            }
            _ => {
                instructions.push(instruction.clone());
            }
        }
    }

    //    print_instructions(&instructions);

    Ok(instructions)
}

fn print_instructions(instructions: &[Instruction]) {
    for (line_no, inst) in instructions.iter().enumerate() {
        println!("{:2}: {}", line_no, inst);
    }
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



fn calculate_h(instructions: &[Instruction]) -> Result<ValueType, Error> {
    let mut registers = RegisterBank::new();
    let id_a = RegisterId::from_name('a')?;
    let id_h = RegisterId::from_name('h')?;
    registers[id_a].value = 1;
    let mut pc: isize = 0;
    let instruction_count = instructions.len() as isize;

    while 0 <= pc && pc < instruction_count {
        let instruction = &instructions[pc as usize];
        match *instruction {
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
            Instruction::Jnq {
                ref condition1,
                ref condition2,
                ref offset,
            } => {
                if let Offset { offset } = *offset {
                    let condition1 = condition1.value(&registers);
                    let condition2 = condition2.value(&registers);
                    if condition1 != condition2 {
                        pc = pc + offset;
                    } else {
                        pc += 1;
                    }
                } else {
                    bail!("Trying to execute symbolic jump {}", offset)
                }
            }
            Instruction::Jnz {
                ref condition,
                ref offset,
            } => {
                if let Offset { offset } = *offset {
                    let condition = condition.value(&registers);
                    if !condition.is_zero() {
                        pc = pc + offset;
                    } else {
                        pc += 1;
                    }
                } else {
                    bail!("Trying to execute symbolic jump {}", offset)
                }
            }
        }
    }

    Ok(registers[id_h].value)
}

fn run() -> Result<(), Error> {
    let instructions = read_input()?;

    let h_value = calculate_h(&instructions)?;

    println!("{}", h_value);

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
