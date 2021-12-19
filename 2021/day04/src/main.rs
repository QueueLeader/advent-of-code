use std::str::FromStr;
use std::collections::HashSet;

use anyhow::{bail, Context};

const ROWS: usize = 5;
const COLS: usize = 5;

#[derive(Debug, Clone)]
struct Board {
    rows: [HashSet<i32>; ROWS],
    cols: [HashSet<i32>; COLS],
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut rows: [HashSet<i32>; ROWS] = Default::default();
        let mut cols: [HashSet<i32>; COLS] = Default::default();

        for (i, line) in s.lines().enumerate() {
            for (j, num) in line.split_whitespace()
                .map(str::parse::<i32>).enumerate() {
                let num = num?;

                rows.get_mut(i)
                    .with_context(||
                        format!("Board has more than {} rows", ROWS)
                    )?.insert(num);
                cols.get_mut(j)
                    .with_context(||
                        format!("Board has more than {} columns", COLS)
                    )?.insert(num);
            }
        }

        Ok(Board {rows, cols})
    }
}

impl Board {
    fn sum(&self) -> i32 {
        self.rows.iter().fold(0, |acc, set| acc + set.iter().sum::<i32>())
    }

    fn call(&mut self, num: i32) -> bool {
        self.rows.iter_mut()
            .for_each(|set| {set.remove(&num);});
        self.cols.iter_mut()
            .for_each(|set| {set.remove(&num);});

        self.rows.iter()
            .any(|set| set.is_empty())
        || self.cols.iter()
            .any(|set| set.is_empty())
    }
}

fn part1(nums: &[i32], mut boards: Vec<Board>) -> anyhow::Result<i32> {
    for &num in nums {
        for board in &mut boards {
            if board.call(num) {
                return Ok(board.sum() * num);
            }
        }
    }

    bail!("Failed to find a winning board")
}

// Part 2
fn part2(nums: &[i32], mut boards: Vec<Board>) -> anyhow::Result<i32> {
    for &num in nums {
        let board_count = boards.len();
        let mut new_boards = Vec::with_capacity(board_count);

        for mut board in boards.into_iter() {
            if board.call(num) {
                if board_count == 1 {
                    return Ok(board.sum() * num);
                }
            } else {
                new_boards.push(board);
            }
        }
        boards = new_boards;
    }

    bail!("Failed to find a winning board")
}

pub fn main() -> anyhow::Result<()> {
    let mut input = include_str!("../input.txt").split("\n\n");
    let nums = input.next()
        .context("Empty input file")?
        .split(',')
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()?;
    let boards = input
        .map(str::parse::<Board>)
        .collect::<Result<Vec<_>, _>>()?;

    // Part 1
    println!("{}", part1(&nums, boards.clone())?);

    // Part 2
    println!("{}", part2(&nums, boards)?);

    Ok(())
}
