use std::str::FromStr;

use anyhow::{Context, ensure};

type Num = u32;

#[derive(Debug)]
struct Matrix {
    data: Vec<u32>,
    rows: usize,
    cols: usize,
}

impl FromStr for Matrix {
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

impl Matrix {
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut u32> {
        if i < self.rows && j < self.cols {
            self.data.get_mut(i * self.cols + j)
        } else {
            None
        }
    }

    fn flash(&mut self, i: usize, j: usize) -> Num {
        let mut count = 0;

        if let Some(n) = self.get_mut(i, j) {
            if *n >= 9 {
                count = 1;
                *n = 0;

                if i > 0 && j > 0 {
                    count += self.flash(i - 1, j - 1);
                }
                if i > 0 {
                    count += self.flash(i - 1, j);
                    count += self.flash(i - 1, j + 1);
                }
                if j > 0 {
                    count += self.flash(i, j - 1);
                    count += self.flash(i + 1, j - 1);
                }
                count += self.flash(i + 1, j);
                count += self.flash(i, j + 1);
                count += self.flash(i + 1, j + 1);
            } else if *n > 0 {
                *n += 1;
            }
        }

        count
    }

    fn step(&mut self) -> Num {
        let mut count = 0;

        self.data.iter_mut().for_each(|n| *n += 1);
        for i in 0 .. self.rows {
            for j in 0 .. self.cols {
                if let Some(10 .. ) = self.get_mut(i, j) {
                    count += self.flash(i, j);
                }
            }
        }

        count
    }

    fn step_n(&mut self, n: u32) -> Num {
        let mut count = 0;

        for _ in 0 .. n {
            count += self.step();
        }

        count
    }

    fn find_synch(&mut self) -> Num {
        let mut step = 1;
        let max_flashes = (self.rows * self.cols) as Num;

        loop {
            if max_flashes == self.step() {
                break step;
            }

            step += 1;
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let mut input = Matrix::from_str(include_str!("../input.txt"))?;
    
    // Part 1
    let flashes = input.step_n(100);
    println!("{}", flashes);

    // Part 2
    println!("{}", 100 + input.find_synch());

    Ok(())
}
