use crate::arguments::Arguments;
use crate::common::position::Position;
use crate::io::reader;
use anyhow::Result;
use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::BufRead;

pub fn day15(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line
            .chars()
            .map(|c| String::from(c).parse().unwrap())
            .collect::<Vec<usize>>();
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

fn get_max_x(board: &Vec<Vec<usize>>) -> usize {
    board.iter().map(|x| x.len()).max().unwrap()
}

fn get_max_y(board: &Vec<Vec<usize>>) -> usize {
    board.len()
}

#[derive(Debug, Default)]
struct Node {
    visited: bool,
    tentative_distance: usize,
    cost: usize,
    position: Position,
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.tentative_distance.eq(&other.tentative_distance) && self.position.eq(&other.position)
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.tentative_distance
            .partial_cmp(&other.tentative_distance)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tentative_distance.cmp(&other.tentative_distance)
    }
}

#[derive(Debug, Default)]
struct Point {
    position: Position,
    tentative_total: usize,
}

impl Point {
    fn new(position: Position, tentative_total: usize) -> Point {
        Point {
            position,
            tentative_total,
        }
    }
}

impl Eq for Point {}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.tentative_total.eq(&other.tentative_total)
    }
}

impl PartialOrd<Self> for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.tentative_total.partial_cmp(&other.tentative_total)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tentative_total.cmp(&other.tentative_total)
    }
}

// This can definitely be improved...
fn dijkstra2(
    lines: &Vec<Vec<usize>>,
    start: &Position,
    end: &Position,
    max_x: usize,
    max_y: usize,
) -> usize {
    let mut nodes = HashMap::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let position = Position::new(x, y);
            let node = Node {
                position: position.clone(),
                tentative_distance: usize::MAX,
                cost: lines[y][x],
                ..Default::default()
            };
            nodes.insert(position, node);
        }
    }
    let mut heap = BinaryHeap::new();

    nodes
        .entry(start.clone())
        .and_modify(|x| x.tentative_distance = 0);
    heap.push(Reverse(Point::new(start.clone(), 0)));

    loop {
        let current = heap.pop();
        if current.is_none() {
            break;
        }
        let current = current.unwrap().0;
        if current.position.eq(end) {
            break;
        }

        // there may be duplicate positions in the heap, so track visited nodes and skip nodes
        // that were already visited
        // todo: is there a better way to do this while keeping the performance benefits
        //  of the heap?
        if nodes.get(&current.position).unwrap().visited {
            continue;
        }
        nodes
            .entry(current.position.clone())
            .and_modify(|x| x.visited = true);
        let adjacent = current
            .position
            .adjacent(max_x, max_y)
            .into_iter()
            .filter(|p| !nodes.get(p).unwrap().visited)
            .collect_vec();

        for p in adjacent {
            let new_tentative = nodes.get(&current.position).unwrap().tentative_distance
                + nodes.get(&p).unwrap().cost;
            if new_tentative < nodes.get(&p).unwrap().tentative_distance {
                nodes
                    .entry(p.clone())
                    .and_modify(|x| x.tentative_distance = new_tentative);
                heap.push(Reverse(Point::new(p, new_tentative)))
            }
        }
    }

    nodes.get(end).unwrap().tentative_distance
}

fn part1(lines: Vec<Vec<usize>>) -> Result<usize> {
    let max_x = get_max_x(&lines);
    let max_y = get_max_y(&lines);

    let start = Position::new(0, 0);
    let end = Position::new(max_x - 1, max_y - 1);

    Ok(dijkstra2(&lines, &start, &end, max_x, max_y))
}

fn sub_board(original: &Vec<Vec<usize>>, increase_amount: usize) -> Vec<Vec<usize>> {
    let mut sub = Vec::new();

    for y in 0..original.len() {
        sub.push(Vec::new());
        for x in 0..original[y].len() {
            let mut new_value = original[y][x];
            for _ in 0..increase_amount {
                new_value += 1;
                if new_value > 9 {
                    new_value = 1;
                }
            }

            sub[y].push(new_value);
        }
    }

    sub
}

fn part2(lines: Vec<Vec<usize>>) -> Result<usize> {
    let max_y = get_max_y(&lines);

    let mut big_board: Vec<Vec<usize>> = Vec::new();
    for _y in 0..(5 * max_y) {
        big_board.push(Vec::new());
    }

    for big_y in 0..5 {
        for big_x in 0..5 {
            let mut sub_board = sub_board(&lines, big_x + big_y);
            for sub_y in 0..sub_board.len() {
                big_board[(max_y * big_y) + sub_y].append(&mut sub_board[sub_y])
            }
        }
    }

    part1(big_board)
}
