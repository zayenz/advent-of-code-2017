#[macro_use]
extern crate failure;
use failure::{Error, Fail};

use std::{io, process};
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;
use std::iter::Peekable;
use std::slice::Iter;


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Token {
    GroupStart,
    GroupEnd,
    GarbageStart,
    GarbageEnd,
    Separator,
    Escape,
    Char(char),
}

use Token::*;

impl Token {}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '{' => GroupStart,
            '}' => GroupEnd,
            '<' => GarbageStart,
            '>' => GarbageEnd,
            ',' => Separator,
            '!' => Escape,
            _ => Char(c),
        }
    }
}



fn read_input() -> Result<Vec<Token>, Error> {
    let mut input = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if !line.is_empty() {
            for c in line.chars() {
                input.push(c.into());
            }
        }
    }
    Ok(input)
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Tree {
    Group { content: Vec<Tree> },
    Garbage { content: Vec<Token> },
}

use Tree::*;

#[derive(Fail, Debug)]
#[fail(display = "A parse error occurred when reading {:?}: {}", token, message)]
struct ParseError {
    token: Option<Token>,
    message: String,
}

impl ParseError {
    fn wrong_token<S: Into<String>>(token: Token, message: S) -> ParseError {
        ParseError {
            token: Some(token),
            message: message.into(),
        }
    }

    fn end_of_input<S: Into<String>>(message: S) -> ParseError {
        ParseError {
            token: None,
            message: message.into(),
        }
    }
}

fn parse_garbage(tokens: &mut Peekable<Iter<Token>>) -> Result<Tree, ParseError> {
    match tokens.peek() {
        None => Err(ParseError::end_of_input(
            "Expected start of group, but no more tokens.",
        )),
        Some(&&GarbageStart) => {
            tokens.next();
            let mut members = Vec::new();
            loop {
                match tokens.peek() {
                    None => {
                        return Err(ParseError::end_of_input(
                            "Parsing group contents, but no more tokens.",
                        ));
                    }
                    Some(&&GarbageEnd) => {
                        tokens.next();
                        break;
                    }
                    Some(&&Escape) => {
                        tokens.next();
                        tokens.next();
                    }
                    Some(_) => {
                        members.push(*tokens.next().unwrap());
                    }
                }
            }
            Ok(Garbage { content: members })
        }
        Some(&&token) => Err(ParseError::wrong_token(token, "Expected start of garbage.")),
    }
}

fn parse_group(tokens: &mut Peekable<Iter<Token>>) -> Result<Tree, ParseError> {
    match tokens.peek() {
        None => Err(ParseError::end_of_input(
            "Expected start of group, but no more tokens.",
        )),
        Some(&&GroupStart) => {
            tokens.next();
            let mut members = Vec::new();

            loop {
                match tokens.peek() {
                    None => {
                        return Err(ParseError::end_of_input(
                            "Parsing group, but no more tokens.",
                        ))
                    }
                    Some(&&GroupStart) => {
                        let group = parse_group(tokens)?;
                        members.push(group);
                    }
                    Some(&&Separator) => {
                        tokens.next();
                    }
                    Some(&&GroupEnd) => {
                        tokens.next();
                        break;
                    }
                    Some(&&GarbageStart) => {
                        let garbage = parse_garbage(tokens)?;
                        members.push(garbage);
                    }
                    Some(&&token) => {
                        return Err(ParseError::wrong_token(
                            token,
                            "Wrong token while parsing group contents.",
                        ))
                    }
                }
            }

            Ok(Group { content: members })
        }
        Some(&&GarbageStart) => parse_garbage(tokens),
        Some(&&token) => Err(ParseError::wrong_token(token, "Expected start of group.")),
    }
}

fn parse(tokens: &mut Peekable<Iter<Token>>) -> Result<Tree, ParseError> {
    parse_group(tokens)
}


fn calculate_score(tree: &Tree) -> i32 {
    match *tree {
        Group { ref content } => {
            let sub_tree_score: i32 = content
                .iter()
                .map(|sub_tree| calculate_score(sub_tree))
                .sum();
            sub_tree_score
        }
        Garbage { ref content } => content.len() as i32,
    }
}

fn run() -> Result<(), Error> {
    let tokens = read_input()?;

    let mut token_iterator: Peekable<Iter<Token>> = tokens.iter().peekable();
    let tree = parse(&mut token_iterator)?;
    let score = calculate_score(&tree);

    println!("{:?}", score);

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
