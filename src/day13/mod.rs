use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use itertools::Itertools;
use std::io::BufRead;

pub fn day13(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut section = 0;
    let mut positions = Vec::new();
    let mut folds = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            section += 1;
            continue;
        }
        if section == 0 {
            positions.push(Position::from_line(line));
        } else {
            folds.push(Fold::from_line(line)?)
        }
    }
    let positions = positions;
    let folds = folds;

    let result = if args.part == 1 {
        part1(positions, folds)
    } else {
        part2(positions, folds)
    }?;

    info!("{:?}", result);

    Ok(())
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    fn from_line(line: String) -> Position {
        let mut parts = line
            .split(",")
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        Position {
            x: parts.remove(0),
            y: parts.remove(0),
        }
    }
}

enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

impl Fold {
    fn from_line(line: String) -> Result<Fold> {
        let line = line.replace("fold along ", "");
        let parts = line.split("=").into_iter().collect_vec();

        match *parts.get(0).unwrap() {
            "x" => Ok(Fold::Vertical(parts.get(1).unwrap().parse().unwrap())),
            "y" => Ok(Fold::Horizontal(parts.get(1).unwrap().parse().unwrap())),
            _ => Err(anyhow::Error::msg("Invalid fold")),
        }
    }
}

fn fold_position(p: &Position, fold: &Fold) -> Position {
    match fold {
        Fold::Horizontal(y) => {
            if p.y < *y {
                p.to_owned()
            } else {
                Position::new(p.x, y - (p.y - y))
            }
        }
        Fold::Vertical(x) => {
            if p.x < *x {
                p.to_owned()
            } else {
                Position::new(x - (p.x - x), p.y)
            }
        }
    }
}

fn fold_once(positions: Vec<Position>, fold: &Fold) -> Vec<Position> {
    positions
        .into_iter()
        .map(|p| fold_position(&p, fold))
        .unique()
        .collect()
}

fn fold(positions: Vec<Position>, folds: &[Fold]) -> Vec<Position> {
    let mut positions = positions;
    for f in folds {
        positions = fold_once(positions, f);
    }

    positions
}

fn part1(positions: Vec<Position>, folds: Vec<Fold>) -> Result<usize> {
    let positions = fold_once(positions, folds.first().unwrap());
    Ok(positions.len())
}

fn part2(positions: Vec<Position>, folds: Vec<Fold>) -> Result<usize> {
    // fold everything
    let positions = fold(positions, &folds);

    // Display the result
    let max_x = positions.iter().map(|p| p.x).max().unwrap();
    let max_y = positions.iter().map(|p| p.y).max().unwrap();

    let mut result: Vec<Vec<String>> = Vec::new();
    for y in 0..max_y + 1 {
        result.push(Vec::new());
        for _x in 0..max_x + 1 {
            result[y].push(String::from("."));
        }
    }

    for p in positions {
        result[p.y][p.x] = String::from("#");
    }

    for row in result {
        info!("{}", row.join(""));
    }

    // This isn't used this time.
    Ok(0)
}
