use crate::arguments::Arguments;
use crate::common::position::Position;
use crate::io::reader;
use anyhow::Result;
use itertools::{min, Itertools};
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
    difficulty: usize,
    position: Position,
}

fn dijkstra(
    lines: &Vec<Vec<usize>>,
    start: &Position,
    end: &Position,
    max_x: usize,
    max_y: usize,
) -> usize {
    let mut nodes = Vec::new();
    let mut unvisited: HashSet<Position> = HashSet::new();
    for y in 0..lines.len() {
        nodes.push(Vec::new());
        for x in 0..lines[y].len() {
            let position = Position::new(x, y);
            let node = Node {
                position: position.clone(),
                tentative_distance: usize::MAX,
                difficulty: lines[y][x].into(),
                ..Default::default()
            };
            unvisited.insert(position);
            nodes[y].push(node);
        }
    }
    nodes[start.y][start.x].tentative_distance = 0;

    let mut current = start.clone();

    loop {
        let adjacent = current
            .adjacent(max_x, max_y)
            .into_iter()
            .filter(|p| unvisited.contains(p))
            .collect_vec();

        for p in adjacent {
            let new_tentative =
                nodes[current.y][current.x].tentative_distance + nodes[p.y][p.x].difficulty;
            if new_tentative < nodes[p.y][p.x].tentative_distance {
                nodes[p.y][p.x].tentative_distance = new_tentative;
            }
        }

        nodes[current.y][current.x].visited = true;
        unvisited.remove(&current);

        if nodes[end.y][end.x].visited {
            break;
        } else {
            let mut min_next = usize::MAX;
            let mut next = None;
            for p in unvisited.iter() {
                if nodes[p.y][p.x].tentative_distance < min_next {
                    min_next = nodes[p.y][p.x].tentative_distance;
                    next = Some(p)
                }
            }
            current = next.unwrap().clone();
        }
    }

    nodes[end.y][end.x].tentative_distance
}

fn part1(lines: Vec<Vec<usize>>) -> Result<usize> {
    let max_x = get_max_x(&lines);
    let max_y = get_max_y(&lines);

    let start = Position::new(0, 0);
    let end = Position::new(max_x - 1, max_y - 1);

    Ok(dijkstra(&lines, &start, &end, max_x, max_y))
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
    let max_x = get_max_x(&lines);
    let max_y = get_max_y(&lines);

    let mut big_board: Vec<Vec<usize>> = Vec::new();
    for y in 0..(5 * max_y) {
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

    info!(
        "Board has {} nodes",
        get_max_x(&big_board) * get_max_y(&big_board)
    );

    part1(big_board)
}
