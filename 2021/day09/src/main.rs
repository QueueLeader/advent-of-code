use std::str::FromStr;

use anyhow::{Context, ensure};

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

struct Adjacents<'a> {
    matrix: &'a Matrix,
    i: usize,
    j: usize,
}

impl Matrix {
    fn adjacents(&self) -> Adjacents<'_> {
        Adjacents {
            matrix: self,
            i: 0,
            j: 0,
        }
    }

    fn get(&self, i: usize, j: usize) -> Option<u32> {
        if i < self.rows && j < self.cols {
            Some(self.data[i * self.cols + j])
        } else {
            None
        }
    }
}

impl<'a> Iterator for Adjacents<'a> {
    type Item = (usize, usize, u32, Vec<u32>);

    fn next(&mut self) -> Option<Self::Item> {
        let mat = self.matrix;
        if self.i == mat.rows {
            return None;
        }

        // We make sure i and j are always in bounds
        // so unwrapping will not panic
        let num = mat.get(self.i, self.j).unwrap();
        let i = self.i;
        let j = self.j;
        let mut adjs = vec![];

        if self.i > 0 {
            if let Some(n) = mat.get(self.i - 1, self.j) {
                adjs.push(n);
            }
        }
        if let Some(n) = mat.get(self.i + 1, self.j) {
            adjs.push(n);
        }
        if self.j > 0 {
            if let Some(n) = mat.get(self.i, self.j - 1) {
                adjs.push(n);
            }
        }
        if let Some(n) = mat.get(self.i, self.j + 1) {
            adjs.push(n);

            self.j += 1;
        } else {
            self.j = 0;
            self.i += 1;
        }

        Some((i, j, num, adjs))
    }
}

fn get_low(mat: &Matrix) -> Vec<(usize, usize, u32)> {
    let mut lows = vec![];

    for (i, j, n, adj) in mat.adjacents() {
        if adj.iter().all(|&x| x > n) {
            lows.push((i, j, n));
        }
    }

    lows
}

fn get_basin_sizes(mat: &Matrix, lows: &[(usize, usize, u32)]) -> Vec<u32> {
    let mut sizes = vec![];
    let mut checked = vec![false; mat.rows * mat.cols];

    for &(i, j, _) in lows {
        sizes.push(basin_size(mat, i, j, &mut checked));
    }

    sizes
}

fn basin_size(mat: &Matrix, i: usize, j: usize, checked: &mut [bool]) -> u32 {
    let mut size = 0;
    let mut to_check = vec![(i, j)];

    while let Some((i, j)) = to_check.pop() {
        if let Some(n) = mat.get(i, j) {
            if checked[i * mat.cols + j] {
                continue;
            }
            checked[i * mat.cols + j] = true;

            if n < 9 {
                size += 1;
            } else {
                continue;
            }

            if i > 0 {
                to_check.push((i - 1, j));
            }
            if j > 0 {
                to_check.push((i, j - 1));
            }
            to_check.push((i + 1, j));
            to_check.push((i, j + 1));
        }
    }

    size
}

pub fn main() -> anyhow::Result<()> {
    let input = Matrix::from_str(include_str!("../input.txt"))?;
    let lows = get_low(&input);
    
    // Part 1
    println!(
        "{}",
        lows.iter().fold(0, |sum, x| sum + x.2 + 1),
    );

    // Part 2
    let mut sizes = get_basin_sizes(&input, &lows);
    sizes.sort_unstable();
    println!(
        "{}",
        sizes.iter().rev().take(3).product::<u32>(),
    );

    Ok(())
}
