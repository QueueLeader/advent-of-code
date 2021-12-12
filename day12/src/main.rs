use std::str::FromStr;
use std::collections::HashMap;

use anyhow::{Context, bail};

type Num = u32;

#[derive(Debug)]
struct Caves {
    adjacents: Vec<Vec<bool>>,
    data: Vec<(CaveType, String)>,
    max_small: usize,
}

#[derive(Debug)]
enum CaveType {
    Small(usize),
    Big,
}

fn add_cave<'a>(
    map: &mut HashMap<&'a str, usize>,
    data: &mut Vec<(CaveType, String)>,
    small_count: &mut usize,
    name: &'a str,
) -> anyhow::Result<()> {
    if name != "end" && !map.contains_key(name) {
        let cave = if name.chars().all(|c| c.is_ascii_lowercase()) {
            let temp = *small_count;
            *small_count <<= 1;

            CaveType::Small(temp)
        } else if name.chars().all(|c| c.is_ascii_uppercase()) {
            CaveType::Big
        } else {
            bail!("Invalid cave: {}", name)
        };

        map.insert(name, data.len());
        data.push((cave, name.into()));
    }

    anyhow::Ok(())
}

impl FromStr for Caves {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut map = HashMap::new();
        let mut data = vec![];
        let mut smalls = 1;

        add_cave(&mut map, &mut data, &mut 0, "start")?;

        for line in s.lines() {
            let (a, b) = line.split_once('-')
                .with_context(|| format!("Invalid line: {}", line))?;
            add_cave(&mut map, &mut data, &mut smalls, a)?;
            add_cave(&mut map, &mut data, &mut smalls, b)?;
        }

        map.insert("end", data.len());
        data.push((CaveType::Small(0), String::from("end")));

        let count = data.len();
        let mut adjacents = Vec::with_capacity(count);
        adjacents.resize(count, vec![false; count]);

        for line in s.lines() {
            // Unwrap will not panic because we already checked earlier
            let (a, b) = line.split_once('-').unwrap();
            let (a, b) = (map[a], map[b]);

            adjacents[a][b] = true;
            adjacents[b][a] = true;
        }

        Ok(Caves {adjacents, data, max_small: smalls - 1})
    }
}

impl Caves {
    fn paths(&self, curr: usize, mut visited: usize, explore: bool) -> Num {
        let mut sum = 0;

        if let CaveType::Small(n) = self.data[curr].0 {
            visited |= n;
        }

        let adjacent = self.adjacents[curr]
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b { Some(i)} else { None });
        for i in adjacent {
            if i == self.data.len() - 1 {
                sum += 1;
                continue;
            }

            sum += match self.data[i].0 {
                CaveType::Big => self.paths(i, visited, explore),
                CaveType::Small(n) => self.explore(n, i, visited, explore),
            };
        }

        sum
    }

    fn explore(
        &self,
        id: usize,
        curr: usize,
        visited: usize,
        explore: bool,
    ) -> Num {
        if visited & id != id {
            self.paths(curr, visited, explore)
        } else if explore && id != 0 {
            self.paths(curr, visited, false)
        } else {
            0
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let caves: Caves = str::parse(include_str!("../input.txt"))?;

    // Part 1
    println!(
        "{}",
        caves.paths(0, 0, false),
    );

    // Part 2
    println!(
        "{}",
        caves.paths(0, 0, true),
    );

    Ok(())
}
