use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use anyhow::{Context, ensure};

type Num = u32;

#[derive(PartialEq, Eq)]
struct Risk {
    risk: Num,
    pos: (usize, usize),
}

// Invert the results of a compare for Risk so that
// the BinaryHeap starts with the smallest value
impl Ord for Risk {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Risk {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl FromStr for Matrix<Num> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let cols = s.find('\n').unwrap_or_else(|| s.len());
        let mut data = vec![];
        let mut rows = 0;

        for line in s.lines() {
            ensure!(line.len() == cols, "Invalid length for row {}", rows);
            for c in line.chars() {
                data.push(c.to_digit(10)
                    .with_context(|| format!("Invalid digit: {}", c))?
                );
            }
            rows += 1;
        }

        Ok(Matrix {data, rows, cols})
    }
}

impl<T> Matrix<T> {
    fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i < self.rows && j < self.cols {
            self.data.get(i * self.cols + j)
        } else {
            None
        }
    }

    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        if i < self.rows && j < self.cols {
            self.data.get_mut(i * self.cols + j)
        } else {
            None
        }
    }

    fn at(&self, i: usize, j: usize) -> &T {
        if i < self.rows && j < self.cols {
            &self.data[i * self.cols + j]
        } else {
            panic!("Invalid coordinates: ({}, {})", i, j)
        }
    }

    fn set(&mut self, i: usize, j: usize, value: T) {
        self.data[i * self.cols + j] = value;
    }

    fn neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];

        if i > 0 {
            neighbors.push((i - 1, j));
        }
        if j > 0 {
            neighbors.push((i, j - 1));
        }
        if i < self.rows - 1 {
            neighbors.push((i + 1, j));
        }
        if j < self.cols - 1 {
            neighbors.push((i, j + 1));
        }

        neighbors
    }
}

impl Matrix<Num> {
    fn filled(rows: usize, cols: usize, value: Num) -> Self {
        Self {data: vec![value; rows * cols], rows, cols}
    }

    // This function works but assumed that the path could only
    // move down and right.
    #[allow(dead_code)]
    fn paths_down_right(&self) -> Self {
        let mut risks = Self::filled(self.rows, self.cols, 0);

        for j in 1 .. self.cols {
            let risk = risks.at(0, j - 1) + self.at(0, j);
            let curr = risks.get_mut(0, j).unwrap();
            *curr = risk;
        }
        for i in 1 .. self.rows {
            let risk = risks.at(i - 1, 0) + self.at(i, 0);
            let curr = risks.get_mut(i, 0).unwrap();
            *curr = risk;
        }
        for i in 1 .. self.rows {
            for j in 1 .. self.cols {
                let a = risks.get(i - 1, j).copied();
                let b = risks.get(i, j - 1).copied();
                let risk = self.at(i, j);
                let curr = risks.get_mut(i, j).unwrap();

                *curr = if let Some(a) = a {
                    if let Some(b) = b {
                        a.min(b) + risk
                    } else {
                        a + risk
                    }
                } else {
                    b.unwrap() + risk
                }
            }
        }

        risks
    }

    fn paths(&self) -> Num {
        let mut dist = Self::filled(self.rows, self.cols, Num::MAX);

        let mut heap = BinaryHeap::new();

        dist.set(0, 0, 0);
        heap.push(Risk {
            risk: 0,
            pos: (0, 0),
        });

        while let Some(Risk {risk, pos}) = heap.pop() {
            if pos == (self.rows - 1, self.cols - 1) {
                return risk;
            }

            if risk > *dist.at(pos.0, pos.1) {
                continue;
            }

            for neighbor in self.neighbors(pos.0, pos.1) {
                let risk = risk + self.at(neighbor.0, neighbor.1);

                if risk < *dist.at(neighbor.0, neighbor.1) {
                    heap.push(Risk {risk, pos: neighbor});
                    dist.set(neighbor.0, neighbor.1, risk);
                }
            }
        }

        0
    }

    fn multi_map(&self, factor: usize) -> Self {
        let rows = self.rows * factor;
        let cols = self.cols * factor;
        let mut data = Vec::with_capacity(rows * cols);

        for i in 0 .. rows {
            for j in 0 .. cols {
                let old = self.at(i % self.rows, j % self.cols);
                let modifier = ((i / self.rows) + (j / self.cols)) as Num;
                data.push((old + modifier - 1) % 9 + 1);
            }
        }

        Self {data, rows, cols}
    }
}

pub fn main() -> anyhow::Result<()> {
    let input = Matrix::<Num>::from_str(
        include_str!("../input.txt"))?;
    
    // Part 1
    let risk = input.paths();
    println!("{}", risk);

    // Part 2
    let risk = input.multi_map(5).paths();
    println!("{}", risk);
    //println!("{}", risks.at(risks.rows - 1, risks.cols - 1));

    Ok(())
}
