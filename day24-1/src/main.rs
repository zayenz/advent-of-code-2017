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
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::str;
use std::char;
use std::ops::*;
use std::fmt;
use std::cmp::{min, max};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Port {
    side1: usize,
    side2: usize,
}

impl Port {
    fn value(&self) -> usize {
        self.side1 + self.side2
    }

    fn other(&self, pin: usize) -> usize {
        if self.side1 == pin {
            self.side2
        } else if self.side2 == pin {
            self.side1
        } else {
            panic!("{} does not match {}", pin, self)
        }
    }

    fn has_pin(&self, pin: usize) -> bool {
        self.side1 == pin || self.side2 == pin
    }
}

impl FromStr for Port {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut parts = s.split('/');
        let side1 = parts.next().unwrap().parse()?;
        let side2 = parts.next().unwrap().parse()?;
        Ok(Port { side1, side2 })
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.side1, self.side2)
    }
}

fn read_input() -> Result<Vec<Port>, Error> {
    let stdin = io::stdin();
    let mut ports = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        ports.push(line.parse()?)
    }

    Ok(ports)
}

#[derive(Debug, Clone)]
struct Bridge {
    ports: Vec<Port>,
    used_pins: HashSet<Port>,
    next_pin: usize,
    value: usize,
}

impl Bridge {
    fn new(start: Port) -> Bridge {
        let ports = vec![start];
        let next_pin = start.other(0);
        let used_pins = [start].iter().cloned().collect();
        let value = Bridge::compute_value(&ports);
        Bridge {
            ports,
            used_pins,
            next_pin,
            value,
        }
    }

    fn compute_value(ports: &[Port]) -> usize {
        ports.iter().map(Port::value).sum()
    }

    fn is_extension(&self, port: &Port) -> bool {
        port.has_pin(self.next_pin) && !self.used_pins.contains(port)
    }

    fn extend(&self, port: &Port) -> Bridge {
        assert!(
            self.is_extension(port),
            "Bridge {} can not be exended by {}",
            self,
            port
        );
        let mut ports = self.ports.clone();
        ports.push(*port);
        let next_pin = port.other(self.next_pin);
        let mut used_pins = self.used_pins.clone();
        used_pins.insert(*port);
        let value = Bridge::compute_value(&ports);
        Bridge {
            ports,
            used_pins,
            next_pin,
            value,
        }
    }
}

impl fmt::Display for Bridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (num, port) in self.ports.iter().enumerate() {
            write!(f, "{}", port)?;
            if num != self.ports.len() - 1 {
                write!(f, "--")?;
            }
        }
        Ok(())
    }
}

fn build(bridge: Bridge, ports: &[Port]) -> Bridge {
    //    println!("Building on {}, extensions {:?}", bridge,
    //        ports.iter().filter(|p| bridge.is_extension(p)).collect::<Vec<_>>()
    //    );
    ports
        .iter()
        .filter(|p| bridge.is_extension(p))
        .map(|p| build(bridge.extend(p), ports))
        .max_by_key(|b| b.value)
        .unwrap_or(bridge)
        .clone()
}

fn find_bridge(ports: &[Port]) -> Result<Bridge, Error> {
    let start_bridges: Vec<Bridge> = ports
        .iter()
        .filter(|p| p.has_pin(0))
        .map(|p| Bridge::new(*p))
        .collect();

    Ok(
        start_bridges
            .into_iter()
            .map(|b| build(b, ports))
            .max_by_key(|b| b.value)
            .unwrap(),
    )
}

fn run() -> Result<(), Error> {
    let ports = read_input()?;

    let bridge = find_bridge(&ports)?;

    println!("{}", bridge.value);
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
