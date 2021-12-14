use std::str::FromStr;
use std::iter::FromIterator;
use std::collections::HashMap;

use anyhow::{bail, Context, ensure};

type Num = u64;

#[derive(Debug)]
struct Polymer {
    value: String,
    rules: HashMap<String, char>,
}

impl FromStr for Polymer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (template, lines) = s.split_once("\n\n")
            .context("Invalid input")?;
        let mut rules = HashMap::new();

        for line in lines.lines() {
            let (a, b) = line.split_once(" -> ")
                .with_context(|| format!("Invalid rule: {}", line))?;

            ensure!(b.len() == 1, "Invalid rule: {}", line);

            rules.insert(a.into(), b.chars().next()
                .with_context(|| format!("Invalid rule: {}", line))?);
        }

        Ok(Self {value: template.into(), rules})
    }
}

impl Polymer {
    #[allow(dead_code)]
    fn step_n(&mut self, n: Num) -> anyhow::Result<()> {
        for _ in 0 .. n {
            self.step()?;
        }

        anyhow::Ok(())
    }

    fn step(&mut self) -> anyhow::Result<()> {
        let mut i = 0;
        let len = self.value.len();
        let mut new_value = String::with_capacity(len * 2);

        while let Some(s) = self.value.get(i .. i+2) {
            let c = self.rules.get(s)
                .with_context(|| format!("Could not find rule for {}", s))?;
            new_value.push_str(&s[0 .. 1]);
            new_value.push(*c);
            i += 1;
        }

        ensure!(i + 1 == len, "Invalid polymer: {}", self.value);
        new_value.push_str(self.value.get(i .. i+1)
            .with_context(|| format!("Invalid polymer: {}", self.value))?);

        self.value = new_value;

        anyhow::Ok(())
    }

    #[allow(dead_code)]
    fn counts(&self) -> HashMap<char, Num> {
        self.value
            .chars()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            })
    }

    fn counts_for(
        &self,
        a: char,
        b: char,
        depth: Num,
        max: Num,
        table: &mut HashMap<(Num, String), HashMap<char, Num>>
    ) -> anyhow::Result<HashMap<char, Num>> {
        if depth >= max {
            return anyhow::Ok(HashMap::new());
        }

        let ab = String::from_iter([a, b]);

        if let Some(counts) = table.get(&(depth, ab.clone())) {
            return anyhow::Ok(counts.clone());
        }

        let c = self.rules.get(&ab)
            .with_context(|| format!("Could not find rule for {}", ab))?;

        let mut map = HashMap::new();
        *map.entry(*c).or_insert(0) += 1;
        merge(&mut map, &self.counts_for(a,*c, depth+1, max, table)?);
        merge(&mut map, &self.counts_for(*c,b, depth+1, max, table)?);

        table.insert((depth, ab), map.clone());

        anyhow::Ok(map)
    }

    fn counts_after_n(&self, n: Num) -> anyhow::Result<HashMap<char, Num>> {
        let mut map = HashMap::new();
        let mut table = HashMap::new();
        let vec = self.value.chars().collect::<Vec<char>>();

        for arr in vec.windows(2) {
            if let [a, b] = *arr {
                *map.entry(a).or_insert(0) += 1;
                *map.entry(b).or_insert(0) += 1;
                merge(&mut map, &self.counts_for(a, b, 0, n, &mut table)?);
            } else {
                bail!("Invalid polymer: {}", self.value);
            }
        }

        anyhow::Ok(map)
    }
}

fn merge(map1: &mut HashMap<char, Num>, map2: &HashMap<char, Num>) {
    for (k, v) in map2.iter() {
        *map1.entry(*k).or_insert(0) += *v;
    }
}

fn max_min_diff(map: &HashMap<char, Num>) -> Num {
    let mut counts: Vec<Num> = map.values()
        .copied()
        .collect();
    counts.sort_unstable();

    counts[counts.len() - 1] - counts[0]
}

pub fn main() -> anyhow::Result<()> {
    let polymer: Polymer = str::parse(include_str!("../input.txt"))?;

    // Part 1
    let counts = polymer.counts_after_n(10)?;
    println!("{}", max_min_diff(&counts));

    // Part 2
    let counts = polymer.counts_after_n(40)?;
    println!("{}", max_min_diff(&counts));

    Ok(())
}
