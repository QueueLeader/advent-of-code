use std::str::FromStr;

const MAX_TIME: usize = 9;
const NORM_TIME: usize = 7;

type FishNum = u64;

#[derive(Debug, Clone)]
struct Lungfishes([FishNum; MAX_TIME]);

impl FromStr for Lungfishes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(Lungfishes(
            s.trim()
                .split(',')
                .map(str::parse::<usize>)
                .try_fold([0; MAX_TIME], |mut fish, num| {
                    fish[num?] += 1;
                    anyhow::Ok(fish)
                })?
        ))
    }
}

impl Lungfishes {
    fn step_day(&mut self) {
        let spawning = self.0[0];
        self.0.rotate_left(1);
        self.0[NORM_TIME - 1] += spawning;
    }

    fn step_days(&mut self, days: i32) -> FishNum {
        for _ in 0 .. days {
            self.step_day();
        }

        self.sum()
    }

    fn sum(&self) -> FishNum {
        self.0.iter().sum()
    }
}

pub fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");
    let mut fish: Lungfishes = str::parse(input)?;
    
    // Part 1
    println!(
        "{}",
        fish.clone().step_days(80),
    );

    // Part 2
    println!(
        "{}",
        fish.step_days(256),
    );

    Ok(())
}
