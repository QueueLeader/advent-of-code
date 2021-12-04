use std::str::FromStr;
use std::collections::HashSet;
use anyhow::{anyhow, bail};

const ROWS: usize = 5;
const COLS: usize = 5;

#[derive(Debug)]
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
                    .ok_or(anyhow!("Board has more than {} rows", ROWS))?
                    .insert(num);
                cols.get_mut(j)
                    .ok_or(anyhow!("Board has more than {} columns", COLS))?
                    .insert(num);
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

fn part1(input: &str) -> anyhow::Result<i32> {
    let mut input = input.split("\n\n");
    let nums = input.next()
        .ok_or(anyhow!("Empty input file"))?
        .split(',')
        .map(str::parse::<i32>);
    let mut boards = input
        .map(str::parse::<Board>)
        .collect::<Result<Vec<_>, _>>()?;

    for num in nums {
        let num = num?;
        for board in &mut boards {
            if board.call(num) {
                return Ok(board.sum() * num);
            }
        }
    }

    bail!("Failed to find a winning board")
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let mut input = input.split("\n\n");
    let nums = input.next()
        .ok_or(anyhow!("Empty input file"))?
        .split(',')
        .map(str::parse::<i32>);
    let mut boards = input
        .map(str::parse::<Board>)
        .collect::<Result<Vec<_>, _>>()?;

    for num in nums {
        let num = num?;
        let mut winners = vec![];
        let board_count = boards.len();
        for (i, board) in boards.iter_mut().enumerate() {
            if board.call(num) {
                if board_count == 1 {
                    return Ok(board.sum() * num);
                } else {
                    winners.push(i);
                }
            }
        }
        boards = boards.into_iter()
            .enumerate()
            .filter(|(i, _)| !winners.contains(i))
            .map(|(_, b)| b)
            .collect();
    }

    bail!("Failed to find a winning board")
}

pub fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    println!("{}", part1(input)?);
    println!("{}", part2(input)?);

    Ok(())
}
