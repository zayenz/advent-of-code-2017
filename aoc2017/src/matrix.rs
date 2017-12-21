use failure::Error;

use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    data: Vec<bool>,
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Matrix {
        Matrix {
            width,
            height,
            data: vec![false; width * height],
        }
    }

    pub fn count_true(&self) -> usize {
        self.data.iter().filter(|&&v| v).count()
    }


    fn pos(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y < self.height);
        x + (y * self.width)
    }

    pub fn slice(&self, start_x: usize, start_y: usize, width: usize, height: usize) -> Matrix {
        let mut data = Vec::with_capacity(width * height);
        for x in start_x..(start_x + width) {
            for y in start_y..(start_y + height) {
                data.push(self[(x, y)])
            }
        }
        Matrix {
            width,
            height,
            data,
        }
    }

    pub fn fill_from(&mut self, x: usize, y: usize, source: &Matrix) {
        for source_x in 0..source.width {
            for source_y in 0..source.height {
                self[(x + source_x, y + source_y)] = source[(source_x, source_y)];
            }
        }
    }

    pub fn rot90(&self) -> Matrix {
        let mut result = Matrix::new(self.height, self.width);

        for x in 0..self.width {
            for y in 0..self.height {
                let xr90 = y;
                let yr90 = self.height - x - 1;
                result[(xr90, yr90)] = self[(x, y)];
            }
        }

        result
    }

    pub fn flip(&self) -> Matrix {
        let mut result = Matrix::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                result[(self.width - x - 1, y)] = self[(x, y)];
            }
        }

        result
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.data[self.pos(x, y)]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        let position = self.pos(x, y);
        &mut self.data[position]
    }
}

impl FromStr for Matrix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let rows: Vec<&str> = s.split('/').collect();
        if rows.is_empty() {
            return Ok(Matrix::new(0, 0));
        }
        let width = rows[0].len();
        let height = rows.len();
        let mut result = Matrix::new(width, height);
        for (y, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                result[(x, y)] = ch == '#';
            }
        }
        Ok(result)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self[(x, y)] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
