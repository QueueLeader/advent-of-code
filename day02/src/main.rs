use std::str::FromStr;
use anyhow::anyhow;

#[derive(Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut words = s.split(' ');
        let cmd = words.next()
            .ok_or(anyhow!("Missing command"))?;
        let num = words.next()
            .ok_or(anyhow!("Missing number"))?
            .parse::<i32>()?;

        match cmd {
            "forward" => Ok(Self::Forward(num)),
            "up" => Ok(Self::Up(num)),
            "down" => Ok(Self::Down(num)),
            _ => Err(anyhow!("Invalid command: {}", cmd))
        }
    }
}

struct Pos {
    x: i32,
    y: i32,
    aim: i32,
}

impl Pos {
    fn product(&self) -> i32 {
        self.x * self.y
    }

    fn origin() -> Self {
        Self{x: 0, y: 0, aim: 0}
    }
}

// Part 1
fn track_pos(pos: Pos, cmd: &str) -> anyhow::Result<Pos> {
    let cmd = str::parse(cmd)?;

    Ok(match cmd {
        Command::Forward(n) => Pos{x: pos.x + n, ..pos},
        Command::Up(n) => Pos{y: pos.y - n, ..pos},
        Command::Down(n) => Pos{y: pos.y + n, ..pos},
    })
}

// Part 2
fn track_pos_with_aim(pos: Pos, cmd: &str) -> anyhow::Result<Pos> {
    let cmd = str::parse(cmd)?;

    Ok(match cmd {
        Command::Forward(n) => Pos{
            x: pos.x + n,
            y: pos.y + n * pos.aim,
            ..pos
        },
        Command::Up(n) => Pos{aim: pos.aim - n, ..pos},
        Command::Down(n) => Pos{aim: pos.aim + n, ..pos},
    })
}

pub fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    // Part 1
    println!(
        "{}",
        input
            .lines()
            .try_fold(Pos::origin(), track_pos)?
            .product(),
    );

    // Part 2
    println!(
        "{}",
        input
            .lines()
            .try_fold(Pos::origin(), track_pos_with_aim)?
            .product(),
    );

    Ok(())
}
