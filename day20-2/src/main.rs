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

type Scalar = i64;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Vector {
    x: Scalar,
    y: Scalar,
    z: Scalar,
}

impl Vector {
    fn new(x: Scalar, y: Scalar, z: Scalar) -> Vector {
        Vector { x, y, z }
    }

    fn manhattan(&self) -> Scalar {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl FromStr for Vector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().trim_left_matches('<').trim_right_matches('>');
        let xyz: Vec<&str> = parts.split(',').map(|s| s.trim()).collect();
        ensure!(
            xyz.len() == 3,
            "Number f components in \"{}\" should be 3, not {}",
            s,
            xyz.len()
        );
        Ok(Vector {
            x: xyz[0].parse()?,
            y: xyz[1].parse()?,
            z: xyz[2].parse()?,
        })
    }
}


impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{},{},{}>", self.x, self.y, self.z)
    }
}



#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl Particle {
    fn new(position: Vector, velocity: Vector, acceleration: Vector) -> Particle {
        Particle {
            position,
            velocity,
            acceleration,
        }
    }

    fn step_in_place(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }

    fn step(&self) -> Particle {
        let mut particle = *self;
        particle.step_in_place();
        particle
    }
}

impl FromStr for Particle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(">,").collect();
        fn as_vector(s: &str) -> Result<Vector, Error> {
            Ok(s.split('=').nth(1).unwrap().parse()?)
        }
        Ok(Particle {
            position: as_vector(parts[0])?,
            velocity: as_vector(parts[1])?,
            acceleration: as_vector(parts[2])?,
        })
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p={}, v={}, a={}",
            self.position,
            self.velocity,
            self.acceleration
        )
    }
}


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NamedParticle {
    id: usize,
    particle: Particle,
}

impl NamedParticle {
    fn new(id: usize, particle: Particle) -> NamedParticle {
        NamedParticle { id, particle }
    }
}

impl fmt::Display for NamedParticle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.id, self.particle)
    }
}

fn read_input() -> Result<Vec<NamedParticle>, Error> {
    let mut particles = Vec::new();
    let mut index = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if !line.is_empty() {
            particles.push(NamedParticle::new(index, line.parse()?));
            index += 1;
        }
    }

    Ok(particles)
}



fn run() -> Result<(), Error> {
    let mut particles = read_input()?;

    const ITERATIONS: usize = 10_000;

    for _ in 0..ITERATIONS {
        let mut seen = HashSet::new();
        let mut collisions = HashSet::new();

        for np in &mut particles {
            np.particle.step_in_place();
            let position = np.particle.position;
            if !seen.insert(position) {
                collisions.insert(position);
            }
        }

        particles = particles
            .into_iter()
            .filter(|np| !collisions.contains(&np.particle.position))
            .collect();
    }

    println!("{}", particles.len());
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
