use anyhow::{bail, Context};

// Part 1
#[derive(Debug, Clone)]
enum BinaryMajority {
    One(u32),
    Zero(u32),
}

impl From<&BinaryMajority> for i32 {
    fn from(item: &BinaryMajority) -> Self {
        match item {
            BinaryMajority::One(_) => 1,
            BinaryMajority::Zero(_) => 0,
        }
    }
}


impl BinaryMajority {
    fn count_binary(&mut self, num: char) -> anyhow::Result<()> {
        *self = match num {
            '1' => self.count_one(),
            '0' => self.count_zero(),
            _ => bail!("Unknown character: {}", num)
        };

        Ok(())
    }

    fn count_zero(&self) -> Self {
        match self {
            Self::One(0) => Self::Zero(1),
            Self::Zero(n) => Self::Zero(n + 1),
            Self::One(n) => Self::One(n - 1),
        }
    }

    fn count_one(&self) -> Self {
        match self {
            Self::Zero(0) => Self::One(1),
            Self::Zero(n) => Self::Zero(n - 1),
            Self::One(n) => Self::One(n + 1),
        }
    }
}

#[derive(Debug)]
struct Digits(Vec<BinaryMajority>);

impl From<Digits> for i32 {
    fn from(item: Digits) -> Self {
        item.0.iter()
            .fold(0, |acc, num| acc * 2 + Self::from(num))
    }
}

impl Digits {
    fn with(size: usize) -> Self {
        Digits(vec![BinaryMajority::Zero(0); size])
    }

    fn product(self) -> i32 {
        let len = self.0.len();
        let num: i32 = self.into();

        num * (num ^ ((2 << (len - 1)) - 1))
    }
}

fn count_ones(mut ones: Digits, binary: &str) -> anyhow::Result<Digits> {
    binary.char_indices().try_for_each(|(i, n)| {
        ones.0[i].count_binary(n)
    })?;

    Ok(ones)
}

// Part 2
fn bit_criteria(input: &str, common: bool) -> anyhow::Result<i32> {
    let digits = input.find('\n').unwrap_or(input.len());
    let mut lines: Vec<&str> = input.lines().collect();

    for i in 0 .. digits {
        let count = lines.len();
        let mut ones = 0;

        if count == 1 {
            break;
        }

        for line in &lines {
            if line.chars().nth(i)
                .with_context(||
                    format!("Could not get char {} from \"{}\"", i, line)
                )? == '1' {
                ones += 1;
            }
        }

        let crit = if (ones * 2 >= count) == common { '1' } else { '0' };
        lines.retain(
            // Unwrap will not panic because we checked in the previous
            // for loop and would return an error if that failed
            |l| l.chars().nth(i).unwrap() == crit
            );
    }

    Ok(i32::from_str_radix(lines[0], 2)?)
}

pub fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");
    let digits = input.find('\n').unwrap_or(input.len());

    // Part 1
    println!(
        "{}",
        input
            .lines()
            .try_fold(Digits::with(digits), count_ones)?
            .product(),
    );

    // Part 2
    println!(
        "{}",
        bit_criteria(input, true)?
        * bit_criteria(input, false)?,
    );

    Ok(())
}
