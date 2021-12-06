use std::str::FromStr;
use std::collections::HashMap;

use anyhow::Context;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pair {
    x: i32,
    y: i32,
}

struct PointIter<'a> {
    line: &'a Line,
    n: i32,
	done: bool,
}

impl Iterator for PointIter<'_> {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let x = self.line.start.x + self.line.slope.x * self.n;
        let y = self.line.start.y + self.line.slope.y * self.n;

        let out = Pair{x, y};

        if out == self.line.end {
            self.done = true;
        }
        self.n += 1;

        Some(out)
    }
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
	// Use Euclid's algorithm
	while m != 0 {
		let temp = m;
		m = n % temp;
		n = temp;
	}
	n.abs()
}

impl Pair {
    fn slope(&self, end: &Self) -> Self {
        let x = end.x - self.x;
        let y = end.y - self.y;

        let gcd = gcd(x, y);

        Pair{x: x / gcd, y: y / gcd}
    }
}

#[derive(Debug)]
struct Line {
    start: Pair,
    end: Pair,
    slope: Pair,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (s1, s2) = s.split_once(" -> ").context("Invalid line")?;

        let parse_pair = |s: &str| -> anyhow::Result<Pair> {
            let (x,y) = s.split_once(',').context("Invalid point")?;
            Ok(Pair{x: x.parse()?, y: y.parse()?})
        };

        let start = parse_pair(s1)?;
        let end = parse_pair(s2)?;

        let slope = start.slope(&end);

        Ok(Line{start, end, slope})
    }
}

impl Line {
    fn is_straight(&self) -> bool {
        self.slope.x == 0 || self.slope.y == 0
    }

    fn points(&self) -> PointIter {
        PointIter{line: self, n: 0, done: false}
    }
}

fn count_overlaps<P>(
    lines: &[Line],
    map: &mut HashMap<Pair, i32>,
    predicate: P
) -> usize
where
    P: FnMut(&&Line) -> bool,
{
    for line in lines.iter().filter(predicate) {
        for point in line.points() {
            *map.entry(point).or_insert(0) += 1;
        }
    }

    map.iter()
        .filter(|(_, v)| **v >= 2)
        .count()
}

pub fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");
    let lines = input.lines()
        .map(str::parse::<Line>)
        .collect::<Result<Vec<_>, _>>()?;
    
    let mut map = HashMap::new();

    // Part 1
    println!(
        "{}",
        count_overlaps(&lines, &mut map, |l| l.is_straight()),
    );

    // Part 2
    println!(
        "{}",
        count_overlaps(&lines, &mut map, |l| !l.is_straight()),
    );

    Ok(())
}
