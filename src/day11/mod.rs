use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::io::BufRead;

pub fn day11(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line
            .chars()
            .map(|c| Octopus::new(String::from(c).parse().unwrap()))
            .collect::<Vec<Octopus>>();
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
struct Octopus {
    energy: u64,
    flashed: bool,
}

impl Octopus {
    fn new(energy: u64) -> Octopus {
        Octopus {
            energy,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

fn increase_energy(lines: &mut Vec<Vec<Octopus>>) {
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            lines[y][x].energy += 1;
        }
    }
}

fn adjacent_octopods(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<Position> {
    let mut adjacent = Vec::new();

    /*
    x-1,y-1     x,y-1       x+1,y-1
    x-1,y       ---         x+1,y
    x-1,y+1     x,y+1       x+1,y+1
    */
    if x != 0 && y != 0 {
        adjacent.push(Position::new(x - 1, y - 1));
    }
    if y != 0 {
        adjacent.push(Position::new(x, y - 1));
    }
    if x + 1 < x_max && y != 0 {
        adjacent.push(Position::new(x + 1, y - 1));
    }
    if x != 0 {
        adjacent.push(Position::new(x - 1, y));
    }
    if x + 1 < x_max {
        adjacent.push(Position::new(x + 1, y));
    }
    if x != 0 && y + 1 < y_max {
        adjacent.push(Position::new(x - 1, y + 1));
    }
    if y + 1 < y_max {
        adjacent.push(Position::new(x, y + 1));
    }
    if x + 1 < x_max && y + 1 < y_max {
        adjacent.push(Position::new(x + 1, y + 1));
    }

    adjacent
}

const FLASH_THRESHOLD: u64 = 10;

fn flash_octopus(lines: &mut Vec<Vec<Octopus>>, p: Position) {
    let mut octopus = lines[p.y].get_mut(p.x).unwrap();

    if octopus.energy < FLASH_THRESHOLD {
        return;
    }
    if octopus.flashed {
        return;
    }
    if octopus.energy >= FLASH_THRESHOLD {
        octopus.flashed = true;
    }

    let adjacent = adjacent_octopods(p.x, p.y, lines[p.y].len(), lines.len());
    for adjacent_p in adjacent {
        let mut adjacent_o = lines[adjacent_p.y].get_mut(adjacent_p.x).unwrap();
        adjacent_o.energy += 1;
        flash_octopus(lines, adjacent_p);
    }
}

fn flash(lines: &mut Vec<Vec<Octopus>>) {
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            flash_octopus(lines, Position::new(x, y));
        }
    }
}

fn find_flashed(lines: &mut Vec<Vec<Octopus>>) -> Vec<Position> {
    let mut flashed = Vec::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x].flashed {
                flashed.push(Position::new(x, y));
            }
        }
    }
    flashed
}

fn reset_flashed(lines: &mut Vec<Vec<Octopus>>, flashed: &[Position]) {
    for p in flashed {
        let mut o = lines[p.y].get_mut(p.x).unwrap();
        o.energy = 0;
        o.flashed = false;
    }
}

fn step(lines: &mut Vec<Vec<Octopus>>) -> Vec<Position> {
    increase_energy(lines);
    flash(lines);
    let flashed = find_flashed(lines);
    reset_flashed(lines, &flashed);
    flashed
}

fn part1(mut lines: Vec<Vec<Octopus>>) -> Result<usize> {
    let mut count = 0;
    for _ in 0..100 {
        let flashed = step(&mut lines);
        count += flashed.len();
    }
    Ok(count)
}

fn part2(mut lines: Vec<Vec<Octopus>>) -> Result<usize> {
    let num_octopods = lines.iter().flat_map(|line| line.iter()).count();
    for i in 0..1000 {
        let flashed = step(&mut lines);
        if flashed.len() == num_octopods {
            return Ok(i + 1);
        }
    }
    Err(anyhow::Error::msg("Did not sync"))
}
