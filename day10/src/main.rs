use anyhow::bail;

type Num = u64;

#[derive(Debug)]
enum Symbol {
    Open(SymbolType),
    Close(SymbolType),
}

#[derive(Debug, PartialEq)]
enum SymbolType {
    Parenthesis,
    SquareBracket,
    CurlyBrace,
    AngleBracket,
}

use self::{Symbol::*, SymbolType::*};

impl Symbol {
    fn from_char(c: char) -> anyhow::Result<Self> {
        Ok(match c {
            '(' => Open(Parenthesis),
            '[' => Open(SquareBracket),
            '{' => Open(CurlyBrace),
            '<' => Open(AngleBracket),
            ')' => Close(Parenthesis),
            ']' => Close(SquareBracket),
            '}' => Close(CurlyBrace),
            '>' => Close(AngleBracket),
            _   => bail!("Invalid character: {}", c)
        })
    }
}

impl SymbolType {
    fn score(&self) -> Num {
        match *self {
            Parenthesis => 3,
            SquareBracket => 57,
            CurlyBrace => 1197,
            AngleBracket => 25137,
        }
    }
}

#[derive(Debug)]
enum Chunk {
    Corrupt(SymbolType),
    Incomplete(Vec<SymbolType>),
    Complete,
}

impl Chunk {
    fn score(&self) -> Num {
        match self {
            Self::Corrupt(a) => a.score(),
            Self::Incomplete(symbols) =>
                symbols.iter()
                    .rev()
                    .fold(0, |sum, sym|
                        sum * 5 + match *sym {
                            Parenthesis => 1,
                            SquareBracket => 2,
                            CurlyBrace => 3,
                            AngleBracket => 4,
                        }
                    ),
            _ => 0
        }
    }

    fn is_corrupt(&self) -> bool {
        matches!(self, Self::Corrupt(_))
    }

    fn is_incomplete(&self) -> bool {
        matches!(self, Self::Incomplete(_))
    }
}

fn check_chunk(line: &str) -> anyhow::Result<Chunk> {
    let symbols = line.chars()
        .map(Symbol::from_char)
        .collect::<Result<Vec<_>, _>>()?;
    let mut stack = vec![];

    for sym in symbols {
        match sym {
            Open(a) => stack.push(a),
            Close(a) =>
                if let Some(b) = stack.pop() {
                    if b != a {
                        return Ok(Chunk::Corrupt(a));
                    }
                } else {
                    return Ok(Chunk::Corrupt(a));
                },
        }
    }

    Ok(
        if stack.is_empty() {
            Chunk::Complete
        } else {
            Chunk::Incomplete(stack)
        }
    )
}

pub fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt")
        .lines()
        .map(check_chunk)
        .collect::<Result<Vec<_>, _>>()?;
    
    // Part 1
    println!(
        "{}",
        input.iter()
            .filter(|chunk| chunk.is_corrupt())
            .fold(0, |sum, chunk| sum + chunk.score()),
    );

    // Part 2
    let mut scores: Vec<_> = input.iter()
        .filter(|chunk| chunk.is_incomplete())
        .map(|chunk| chunk.score())
        .collect();
    let len = scores.len();
    println!(
        "{}",
        scores.select_nth_unstable(len / 2).1,
    );

    Ok(())
}
