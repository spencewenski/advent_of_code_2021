use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::BufRead;

pub fn day5(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines: Vec<LineSegment> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = LineSegment::from_str(&line);
        lines.push(line);
    }
    let lines = lines;

    let result = if args.part == 1 {
        part1(lines)
    } else {
        part2(lines)
    }?;

    info!("{}", result);

    Ok(())
}

#[derive(Debug, Default, Eq, PartialEq, Hash)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x, y }
    }

    fn from_str(s: &str) -> Coordinate {
        let nums: Vec<u32> = s.split(",").map(|x| x.trim().parse().unwrap()).collect();

        Coordinate {
            x: *nums.get(0).unwrap(),
            y: *nums.get(1).unwrap(),
        }
    }
}

#[derive(Debug, Default)]
struct LineSegment {
    start: Coordinate,
    end: Coordinate,
}

impl LineSegment {
    fn from_str(line: &str) -> LineSegment {
        let mut coords: Vec<Coordinate> = line
            .split("->")
            .into_iter()
            .map(|x| x.trim())
            .map(|x| Coordinate::from_str(x))
            .collect();

        LineSegment {
            start: coords.remove(0),
            end: coords.remove(0),
        }
    }

    // There's probably a better way to do this, but we'll just brute force the solution for now
    fn all_coordinates(&self) -> Vec<Coordinate> {
        let mut coords = Vec::new();
        if self.start.x == self.end.x {
            // vertical
            for y in min(self.start.y, self.end.y)..(max(self.start.y, self.end.y) + 1) {
                coords.push(Coordinate::new(self.start.x, y));
            }
        } else if self.start.y == self.end.y {
            // horizontal
            for x in min(self.start.x, self.end.x)..(max(self.start.x, self.end.x) + 1) {
                coords.push(Coordinate::new(x, self.start.y));
            }
        } else {
            // diagonal
            let x_increase = self.start.x < self.end.x;
            let y_increase = self.start.y < self.end.y;

            for i in 0..((max(self.start.x, self.end.x) - min(self.start.x, self.end.x)) + 1) {
                let x = if x_increase {
                    self.start.x + i
                } else {
                    self.start.x - i
                };
                let y = if y_increase {
                    self.start.y + i
                } else {
                    self.start.y - i
                };

                coords.push(Coordinate::new(x, y));
            }
        }
        coords
    }
}

fn count_overlaps(lines: Vec<LineSegment>) -> Result<usize> {
    let coords: Vec<Coordinate> = lines
        .into_iter()
        .flat_map(|line| line.all_coordinates())
        .collect();

    let mut counts: HashMap<Coordinate, usize> = HashMap::new();

    for coord in coords {
        let counter = counts.entry(coord).or_insert(0);
        *counter += 1;
    }

    let mut num_overlaps = 0;
    for count in counts.values() {
        if *count > 1 {
            num_overlaps += 1;
        }
    }

    Ok(num_overlaps)
}

fn part1(lines: Vec<LineSegment>) -> Result<usize> {
    let lines = lines
        .into_iter()
        // only consider vertical or horizontal lines
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .collect();

    count_overlaps(lines)
}

fn part2(lines: Vec<LineSegment>) -> Result<usize> {
    count_overlaps(lines)
}
