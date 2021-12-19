use std::collections::HashSet;
use std::iter::FromIterator;

use anyhow::{bail, Context};

fn set2str(set: HashSet<char>) -> String {
    set.iter().fold(String::new(), |mut s, c| {s.push(*c); s})
}

fn arr2num(arr: &[i32]) -> i32 {
    arr.iter().fold(0, |acc, n| acc * 10 + n)
}

fn get_signals(signals: &str) -> anyhow::Result<[HashSet<char>; 10]> {
    let mut known: [HashSet<char>; 10] = Default::default();
    let mut unknown = Vec::with_capacity(10);

    // Prepare HashSets and find 1, 4, 7, and 8
    for signal in signals.split(' ') {
        match signal.len() {
            2 => known[1].extend(signal.chars()),
            3 => known[7].extend(signal.chars()),
            4 => known[4].extend(signal.chars()),
            7 => known[8].extend(signal.chars()),
            _ => unknown.push(HashSet::from_iter(signal.chars()))
        }
    }

    let segs_in_2: HashSet<_> = known[8].difference(&known[4])
        .copied().collect();

    for signal in unknown.into_iter() {
        let sig_len = signal.len();

        if sig_len == 5 {
            if segs_in_2.is_subset(&signal) {
                known[2] = signal;
            } else if known[1].is_subset(&signal) {
                known[3] = signal;
            } else {
                known[5] = signal;
            }
        } else if sig_len == 6 {
            if known[4].is_subset(&signal) {
                known[9] = signal;
            } else if known[1].is_subset(&signal) {
                known[0] = signal;
            } else {
                known[6] = signal;
            }
        } else {
            bail!("Invalid signal: {}", set2str(signal));
        }
    }

    Ok(known)
}

fn get_nums(line: &str) -> anyhow::Result<Vec<i32>> {
    let (signals, encoded) = line.split_once(" | ")
        .context("Invalid input: Missing '|'")?;
    let known = get_signals(signals)?;

    let mut nums = vec![];

    let sets = encoded.split(' ')
        .map(|s| HashSet::from_iter(s.chars()));
    for set in sets {
        nums.push(known.iter()
            .position(|s| s == &set)
            .with_context(||
                format!("Unknown signal: {}", set2str(set))
            )? as i32);
    }

    Ok(nums)
}

pub fn main() -> anyhow::Result<()> {
    let nums = include_str!("../input.txt")
        .lines()
        .map(get_nums)
        .collect::<Result<Vec<_>, _>>()?;
    
    // Part 1
    println!(
        "{}",
        nums.iter()
            .flatten()
            .filter(|n| [1, 4, 7, 8].contains(n))
            .count(),
    );

    // Part 2
    println!(
        "{}",
        nums.iter()
            .map(|arr| arr2num(&arr[..]))
            .sum::<i32>(),
    );

    Ok(())
}
