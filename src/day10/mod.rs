use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;

pub fn day10(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.chars().collect();
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

#[derive(Debug)]
enum SyntaxError {
    None,
    IllegalCharacter(char),
    IncompleteLine(Vec<char> /*the stack*/),
}

fn is_open_char(c: char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

fn is_close_char(c: char) -> bool {
    match c {
        ')' => true,
        ']' => true,
        '}' => true,
        '>' => true,
        _ => false,
    }
}

fn open_char_for_close_char(close_char: char) -> char {
    match close_char {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Illegal char {}", close_char),
    }
}

fn find_syntax_errors(line: &[char]) -> SyntaxError {
    let mut s: Vec<char> = Vec::new();

    for c in line {
        if is_open_char(*c) {
            s.push(*c);
            continue;
        }
        if is_close_char(*c) {
            let last = s.pop().unwrap();
            if last == open_char_for_close_char(*c) {
                continue;
            } else {
                return SyntaxError::IllegalCharacter(*c);
            }
        }
    }

    if !s.is_empty() {
        SyntaxError::IncompleteLine(s)
    } else {
        SyntaxError::None
    }
}

fn value_of_illegal_char(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn part1(lines: Vec<Vec<char>>) -> Result<u64> {
    let result = lines
        .iter()
        .map(|line| find_syntax_errors(&line))
        .map(|e| match e {
            SyntaxError::IllegalCharacter(c) => value_of_illegal_char(c),
            _ => 0,
        })
        .fold(0, |accum, value| accum + value);

    Ok(result)
}

/// The input will be an 'open' char because it's coming from the incomplete stack for the line
fn value_of_closing_char(open_char: char) -> u64 {
    match open_char {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn value_of_incomplete_line(stack: &[char]) -> u64 {
    stack
        .iter()
        .rev()
        .fold(0, |accum, c| (accum * 5) + value_of_closing_char(*c))
}

fn part2(lines: Vec<Vec<char>>) -> Result<u64> {
    let line_values = lines
        .iter()
        .map(|line| find_syntax_errors(&line))
        .map(|e| match e {
            SyntaxError::IncompleteLine(stack) => value_of_incomplete_line(&stack),
            _ => 0,
        })
        .filter(|v| *v > 0)
        .sorted()
        .collect::<Vec<u64>>();

    let middle_value = line_values[line_values.len() / 2];

    Ok(middle_value)
}
