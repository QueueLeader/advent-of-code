use anyhow::Context;

type Num = i32;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: Num,
    y: Num,
}

impl Point {
    fn in_range(&self, min: &Point, max: &Point) -> bool {
        self.x >= min.x && self.x <= max.x
            && self.y >= min.y && self.y <= max.y
    }

    fn hit_target(&self, min: &Point, max: &Point) -> bool {
        let mut vel = Point {x: self.x, y: self.y};
        let mut pos = Point {x: 0, y: 0};
        while pos.x <= max.x && pos.y >= min.y {
            pos.x += vel.x;
            pos.y += vel.y;

            if vel.x > 0 {
                vel.x -= 1;
            }
            vel.y -= 1;

            if pos.in_range(min, max) {
                return true;
            }
        }

        false
    }
}

fn parse_area(s: &str) -> anyhow::Result<(Point, Point)> {
    let s = s.trim().strip_prefix("target area: x=")
        .context("Missing start of target area string")?;
    let (x, y) = s.split_once(", y=")
        .context("Missing middle of target area string")?;

    let parse_range = |s: &str| {
        let (s1, s2) = s.split_once("..")
            .with_context(|| format!("Invalid range: {}", s))?;
        anyhow::Ok((str::parse::<Num>(s1)?, str::parse::<Num>(s2)?))
    };

    let x = parse_range(x)?;
    let y = parse_range(y)?;

    anyhow::Ok((
        Point {
            x: x.0.min(x.1),
            y: y.0.min(y.1),
        },
        Point {
            x: x.0.max(x.1),
            y: y.0.max(y.1),
        },
    ))
}

fn get_possible(min: &Point, max: &Point) -> Vec<Point> {
    let v_min = Point {
        x: (((1 + 8 * min.x) as f64).sqrt() as Num - 1) / 2,
        y: min.y,
    };
    let v_max = Point {
        x: max.x,
        y: -min.y,
    };

    let mut vels = vec![];
    for x in v_min.x .. v_max.x + 1 {
        for y in v_min.y .. v_max.y + 1 {
            let curr = Point {x, y};
            if curr.hit_target(min, max) {
                vels.push(curr);
            }
        }
    }

    vels
}

pub fn main() -> anyhow::Result<()> {
    let (min, max) = parse_area(include_str!("../input.txt"))?;

    // Part 1
    println!("{}", min.y * (min.y + 1) / 2);

    // Part 2
    let possible = get_possible(&min, &max);
    println!("{}", possible.len());

    Ok(())
}

