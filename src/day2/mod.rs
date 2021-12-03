use crate::arguments::Arguments;
use crate::day2::Direction::{Down, Forward, Up};
use crate::io::reader;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::BufRead;

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    fn from_str(s: &str) -> anyhow::Result<Direction> {
        match s {
            "forward" => Ok(Forward),
            "up" => Ok(Up),
            "down" => Ok(Down),
            _ => Err(anyhow::Error::msg(format!("Unrecognized direction {}", s))),
        }
    }
}

#[derive(Debug)]
struct Line {
    direction: Direction,
    amount: i64,
}

impl Line {
    fn from_str(s: &str) -> anyhow::Result<Line> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+) (\d+)").unwrap();
        }
        // let RE = Regex::new(r"(\w+) (\d+)").unwrap();

        let capture = RE
            .captures(s)
            .ok_or(anyhow::Error::msg(format!("Unable to parse line: {}", s)))?;

        let line = Line {
            direction: Direction::from_str(
                capture
                    .get(1)
                    .ok_or(anyhow::Error::msg(format!("Missing direction")))?
                    .as_str(),
            )?,
            amount: capture
                .get(2)
                .ok_or(anyhow::Error::msg(format!("Missing amount")))?
                .as_str()
                .parse()?,
        };

        Ok(line)
    }
}

pub fn day2(args: &Arguments) -> anyhow::Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines: Vec<Line> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = Line::from_str(&line)?;
        lines.push(line);
    }
    let lines = lines;

    let result = if args.part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }?;

    info!("{:?}", result);

    Ok(())
}

#[derive(Debug, Default)]
struct Position {
    horizontal: i64,
    vertical: i64,
}

impl Position {
    fn new(horizontal: i64, vertical: i64) -> Position {
        Position {
            horizontal,
            vertical,
        }
    }

    // used for part 1
    fn move_position(self, direction: &Direction, amount: i64) -> Position {
        match direction {
            Forward => Position::new(self.horizontal + amount, self.vertical),
            Up => Position::new(self.horizontal, self.vertical - amount),
            Down => Position::new(self.horizontal, self.vertical + amount),
        }
    }
}

#[derive(Debug, Default)]
struct Submarine {
    position: Position,
    aim: i64,
}

// used for part 2
impl Submarine {
    fn move_submarine(self, amount: i64) -> Submarine {
        let position = Position {
            horizontal: self.position.horizontal + amount,
            vertical: self.position.vertical + (self.aim * amount),
        };

        Submarine {
            aim: self.aim,
            position,
        }
    }

    fn update_aim(self, amount: i64) -> Submarine {
        Submarine {
            position: self.position,
            aim: self.aim + amount,
        }
    }

    fn update_submarine(self, direction: &Direction, amount: i64) -> Submarine {
        match direction {
            Up => self.update_aim(-amount),
            Down => self.update_aim(amount),
            Forward => self.move_submarine(amount),
        }
    }
}

fn part1(lines: Vec<Line>) -> anyhow::Result<i64> {
    let final_position = lines
        .into_iter()
        .fold(Position::default(), |position, line| {
            position.move_position(&line.direction, line.amount)
        });

    Ok(final_position.vertical * final_position.horizontal)
}

fn part2(lines: Vec<Line>) -> anyhow::Result<i64> {
    let final_sub_state = lines.into_iter().fold(Submarine::default(), |sub, line| {
        sub.update_submarine(&line.direction, line.amount)
    });

    Ok(final_sub_state.position.vertical * final_sub_state.position.horizontal)
}
