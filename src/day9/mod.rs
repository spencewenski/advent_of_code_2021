use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::collections::HashSet;
use std::io::BufRead;

pub fn day9(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line
            .chars()
            .into_iter()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
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

#[derive(Debug, Default, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Default)]
struct LowPoint {
    point: Point,
    value: u8,
}

fn adjacent_points(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<Point> {
    let mut points = Vec::new();
    if x > 0 {
        points.push(Point { x: x - 1, y })
    }
    if x < grid[y].len() - 1 {
        points.push(Point { x: x + 1, y })
    }
    if y > 0 {
        points.push(Point { x, y: y - 1 })
    }
    if y < grid.len() - 1 {
        points.push(Point { x, y: y + 1 })
    }
    points
}

fn is_low_point(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let value = grid[y][x];

    !adjacent_points(grid, x, y)
        .iter()
        .any(|point| grid[point.y][point.x] <= value)
}

fn find_low_points(grid: &Vec<Vec<u8>>) -> Vec<LowPoint> {
    let mut low_points = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if is_low_point(grid, x, y) {
                low_points.push(LowPoint {
                    point: Point { x, y },
                    value: grid[y][x],
                })
            }
        }
    }

    low_points
}

#[derive(Debug, Default)]
struct Basin {
    low: LowPoint,
    points: HashSet<Point>,
}

fn explore_basin(grid: &Vec<Vec<u8>>, basin: &mut Basin, x: usize, y: usize) {
    if basin.points.contains(&Point { x, y }) {
        return;
    }
    let value = grid[y][x];
    if value >= 9 {
        return;
    }
    basin.points.insert(Point { x, y });
    let adjacent: Vec<Point> = adjacent_points(grid, x, y)
        .into_iter()
        .filter(|point| !basin.points.contains(point))
        .collect();

    for point in adjacent {
        explore_basin(grid, basin, point.x, point.y);
    }
}

fn part1(grid: Vec<Vec<u8>>) -> Result<usize> {
    let result = find_low_points(&grid)
        .iter()
        .fold(0, |accum, x| accum + (x.value as usize) + 1);
    Ok(result)
}

fn part2(grid: Vec<Vec<u8>>) -> Result<usize> {
    let mut basins: Vec<Basin> = find_low_points(&grid)
        .into_iter()
        .map(|low| Basin {
            low,
            ..Default::default()
        })
        .collect();

    basins
        .iter_mut()
        .for_each(|basin| explore_basin(&grid, basin, basin.low.point.x, basin.low.point.y));

    // we want to sort in reverse to have the largest numbers at the front
    basins.sort_by(|a, b| b.points.len().cmp(&a.points.len()));

    let first = basins.get(0).unwrap();
    let second = basins.get(1).unwrap();
    let third = basins.get(2).unwrap();

    let result = first.points.len() * second.points.len() * third.points.len();

    Ok(result)
}
