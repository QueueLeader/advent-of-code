enum Possible<T> {
    Single(T),
    Pair(T, T),
}

impl<T: Ord + Copy> Possible<T> {
    fn min(&self, func: impl Fn(T) -> T) -> T {
        match &self {
            Possible::Single(x) => func(*x),
            Possible::Pair(x, y) => func(*x).min(func(*y)),
        }
    }
}

fn median(vec: &mut [i32]) -> Possible<i32> {
    let len = vec.len();
    let mid = len / 2;
    let (_, &mut out, _) = vec.select_nth_unstable(mid);

    if len & 1 == 0 {
        let (_, &mut out2, _) = vec.select_nth_unstable(mid + 1);

        Possible::Pair(out, out2)
    } else {
        Possible::Single(out)
    }
}

fn mean(vec: &[i32]) -> Possible<i32> {
    let sum: i32 = vec.iter().sum();
    let len = vec.len() as i32;
    let mean = sum / len;

    if mean * len < sum {
        Possible::Pair(mean, mean + 1)
    } else {
        Possible::Single(mean)
    }
}

fn arith_sum(num: i32) -> i32 {
    num * (num + 1) / 2
}

pub fn main() -> anyhow::Result<()> {
    let mut input = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()?;
    
    // Part 1
    println!(
        "{}",
        median(&mut input).min(|a| {
            input.iter()
                .map(|&x| (x - a).abs())
                .sum::<i32>()
        }),
    );

    // Part 2
    println!(
        "{}",
        mean(&input).min(|a| {
            input.iter()
                .map(|&x| arith_sum((x - a).abs()))
                .sum::<i32>()
        }),
    );

    Ok(())
}
