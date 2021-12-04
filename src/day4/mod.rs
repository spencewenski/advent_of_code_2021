use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::collections::HashMap;
use std::io::BufRead;

pub fn day4(args: &Arguments) -> Result<()> {
    let mut reader = reader(args.src_file.as_ref())?;

    let mut first_line = String::new();
    reader.read_line(&mut first_line);
    let numbers: Vec<usize> = first_line
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut empty_line = String::new();
    reader.read_line(&mut empty_line);

    let mut boards: Vec<Board> = Vec::new();
    while let Ok(board) = Board::new(&mut reader) {
        boards.push(board);
    }

    let result = if args.part == 1 {
        part1(&numbers, &mut boards)
    } else {
        part2(&numbers, &mut boards)
    }?;

    info!("{:?}", result);

    Ok(())
}

fn part1(numbers: &[usize], boards: &mut Vec<Board>) -> Result<usize> {
    for number in numbers {
        for board in &mut *boards {
            if board.mark(*number) {
                // found a winner
                let sum = board.sum_of_unmarked();
                return Ok(sum * number);
            }
        }
    }
    Err(anyhow::Error::msg("Did not find a winning board"))
}

fn part2(numbers: &[usize], boards: &mut Vec<Board>) -> Result<usize> {
    let mut last_number: Option<usize> = None;
    let mut last_board: Option<usize> = None;

    for number in numbers {
        for i in 0..boards.len() {
            let board = boards.get_mut(i).unwrap();
            if board.won {
                // skip this board if it won already
                continue;
            }
            if board.mark(*number) {
                last_number = Some(*number);
                last_board = Some(i)
            }
        }
    }

    if last_number.is_none() || last_board.is_none() {
        return Err(anyhow::Error::msg("Did not find a winning board"));
    }
    let last_number = last_number.unwrap();
    let last_board = last_board.unwrap();

    info!(
        "num: {}, board: {:?}",
        last_number,
        boards.get(last_board).unwrap()
    );

    let sum = boards.get(last_board).unwrap().sum_of_unmarked();
    Ok(sum * last_number)
}

#[derive(Debug, Default)]
struct Board {
    rows: Vec<Row>,
    positions: HashMap<usize, Position>,
    won: bool,
}

impl Board {
    fn new(reader: &mut Box<dyn BufRead>) -> Result<Board> {
        let mut rows = Vec::new();

        let mut line = String::new();
        while let Ok(count) = reader.read_line(&mut line) {
            if count <= 0 {
                return Err(anyhow::Error::msg("done"));
            }
            if line.trim().len() <= 0 {
                break;
            }
            let row = Row::new(&line);
            rows.push(row);
            line.clear()
        }

        let positions = build_positions(&rows);

        let board = Board {
            rows,
            positions,
            ..Default::default()
        };

        Ok(board)
    }

    /// Returns true if marking the number causes this board to win
    fn mark(&mut self, number: usize) -> bool {
        let position = self.positions.get(&number);
        if position.is_none() {
            return false;
        }
        let position = position.unwrap();

        // mark the square
        let row = self.rows.get_mut(position.y).unwrap();
        let square = row.columns.get_mut(position.x).unwrap();
        square.mark();

        let won = self.check_row(position.y) || self.check_column(position.x);
        self.won = self.won || won;

        return self.won;
    }

    fn check_row(&self, y: usize) -> bool {
        let row = self.rows.get(y).unwrap();
        row.check()
    }

    fn check_column(&self, x: usize) -> bool {
        let any_not_marked = self
            .rows
            .iter()
            .map(|row| row.columns.get(x).unwrap())
            .any(|square| !square.marked);

        !any_not_marked
    }

    fn sum_of_unmarked(&self) -> usize {
        self.rows
            .iter()
            .map(|row| &row.columns)
            .flat_map(|x| x.iter())
            .filter(|square| !square.marked)
            .fold(0, |accum, square| accum + square.number)
    }
}

fn build_positions(rows: &[Row]) -> HashMap<usize, Position> {
    let mut positions = HashMap::new();
    for y in 0..rows.len() {
        let row = &rows[y];
        for x in 0..row.columns.len() {
            let square = &row.columns[x];
            positions.insert(square.number, Position::new(x, y));
        }
    }

    positions
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

#[derive(Debug, Default)]
struct Row {
    columns: Vec<Square>,
}

impl Row {
    fn new(line: &str) -> Row {
        let columns = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .map(|num| Square::new(num))
            .collect();

        Row {
            columns,
            ..Default::default()
        }
    }

    fn check(&self) -> bool {
        let any_not_marked = self.columns.iter().any(|square| !square.marked);
        !any_not_marked
    }
}

#[derive(Debug, Default)]
struct Square {
    number: usize,
    marked: bool,
}

impl Square {
    fn new(number: usize) -> Square {
        Square {
            number,
            ..Default::default()
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}
