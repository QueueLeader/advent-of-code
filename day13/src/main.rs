use std::fmt;
use std::str::FromStr;
use std::collections::BTreeSet;

use anyhow::{bail, Context, ensure};

type Num = u32;

#[derive(Debug)]
struct Paper {
    points: BTreeSet<Point>,
    max_width: Num,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Point {
    y: Num,
    x: Num,
}

impl FromStr for Paper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut points = BTreeSet::new();
        let mut max_width = 0;

        for line in s.lines() {
            let (x, y) = line.split_once(',')
                .with_context(|| format!("Invalid point: {}", line))?;
            let x: Num = str::parse(x)?;
            let y: Num = str::parse(y)?;

            points.insert(Point {x, y});

            if x > max_width {
                max_width = x;
            }
        }

        Ok(Self {points, max_width})
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut last_x = 0;
        let mut last_y = 0;
        for point in &self.points {
            if point.y > last_y {
                if !f.fill().is_ascii_whitespace() {
                    for _ in 0 .. point.y - last_y {
                        for _ in 0 .. self.max_width - (last_x - 1) {
                            write!(f, "{}", f.fill())?;
                        }
                        writeln!(f)?;

                        last_x = 0;
                    }
                } else {
                    write!(f, "{:\n<1$}", "", (point.y - last_y) as usize)?;
                }

                last_y = point.y;
                last_x = 0;
            }

            for _ in 0 .. point.x - last_x {
                write!(f, "{}", f.fill())?;
            }
            write!(f, "#")?;

            last_x = point.x + 1;
        }

        fmt::Result::Ok(())
    }
}

#[derive(Debug)]
enum Fold {
    X(Num),
    Y(Num),
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        ensure!(s.starts_with("fold along "), "Invalid fold: {}", s);

        let fold_type = s.get(11 .. 13)
            .with_context(|| format!("Invalid fold: {}", s))?;

        let num: Num = str::parse(s.get(13..)
            .with_context(|| format!("Invalid fold: {}", s))?)?;

        Ok(match fold_type {
            "x=" => Self::X(num),
            "y=" => Self::Y(num),
            _ => bail!("Invalid fold: {}", s)
        })
    }
}

impl Paper {
    fn fold(self, folds: &[Fold]) -> Self {
        let mut points = BTreeSet::new();
        let mut width_change = false;
        let mut max_width = 0;

        let fold_num = |curr, fold_num| {
            if curr > fold_num {
                fold_num * 2 - curr
            } else {
                curr
            }
        };

        for point in self.points.into_iter() {
            points.insert(folds.iter().fold(point, |p, f|
                match f {
                    Fold::X(n) => {
                        let x = fold_num(p.x, *n);
                        if x > max_width {
                            max_width = x;
                            width_change = true;
                        }

                        Point{x, ..p}
                    },
                    Fold::Y(n) => Point{y: fold_num(p.y, *n), ..p},
                }
            ));
        }

        if !width_change {
            max_width = self.max_width;
        }

        Self {points, max_width}
    }
}

pub fn main() -> anyhow::Result<()> {
    let (paper, folds) = include_str!("../input.txt")
        .split_once("\n\n").context("Invalid input")?;
    let paper: Paper = str::parse(paper)?;
    let folds = folds.lines()
        .map(str::parse::<Fold>)
        .collect::<Result<Vec<_>, _>>()?;

    // Part 1
    let paper = paper.fold(&folds[0..1]);
    println!("{}", paper.points.len());

    // Part 2
    let paper = paper.fold(&folds[1..]);
    println!("{}", paper);

    Ok(())
}
