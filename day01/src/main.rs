pub fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    // Part 1
    println!(
        "{}",
        input
            .lines()
            .map(str::parse::<i32>)
            .collect::<Result<Vec<_>, _>>()?
            .windows(2)
            .filter(|depth| depth[1] > depth[0])
            .count(),
    );

    // Part 2
    println!(
        "{}",
        input
            .lines()
            .map(str::parse::<i32>)
            .collect::<Result<Vec<_>, _>>()?
            .windows(4)
            .filter(|depth| depth[3] > depth[0])
            .count(),
    );

    Ok(())
}
