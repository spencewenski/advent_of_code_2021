use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

pub fn day8(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = get_digits_for_line(&line)?;
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

fn get_easy_code(line: &[String], length: usize) -> String {
    line.iter()
        .filter(|s| s.len() == length)
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .first()
        .unwrap()
        .to_owned()
}

fn get_ambiguous_code(line: &[String], length: usize, sub_code: &str, invert: bool) -> String {
    line.iter()
        .filter(|s| s.len() == length)
        .filter(|s| {
            let b = sub_code.chars().all(|c| s.contains(c));
            if invert {
                !b
            } else {
                b
            }
        })
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .first()
        .unwrap()
        .to_owned()
}

fn get_codes(left: Vec<String>) -> HashMap<String, char> {
    let mut codes = HashMap::new();

    let one = get_easy_code(&left, 2);
    let four = get_easy_code(&left, 4);
    let seven = get_easy_code(&left, 3);
    let eight = get_easy_code(&left, 7);

    let four_remainder = four
        .chars()
        .filter(|c| !one.contains(*c))
        .collect::<String>();

    let eight_remainder = eight
        .chars()
        .filter(|c| !seven.contains(*c))
        .filter(|c| !four_remainder.contains(*c))
        .collect::<String>();

    let two = get_ambiguous_code(&left, 5, &eight_remainder, false);
    let three = get_ambiguous_code(&left, 5, &one, false);
    let five = get_ambiguous_code(&left, 5, &four_remainder, false);
    let six = get_ambiguous_code(&left, 6, &one, true);

    codes.insert(one, '1');
    codes.insert(two, '2');
    codes.insert(three, '3');
    codes.insert(four, '4');
    codes.insert(five, '5');
    codes.insert(six, '6');
    codes.insert(seven, '7');
    codes.insert(eight, '8');

    let remaining = left
        .iter()
        .filter(|s| !codes.contains_key(*s))
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let nine = get_ambiguous_code(&remaining, 6, &four_remainder, false);
    codes.insert(nine, '9');

    let zero = get_ambiguous_code(&remaining, 6, &eight_remainder, false);
    codes.insert(zero, '0');

    codes
}

fn parse_to_digits(codes: &HashMap<String, char>, digit_strings: &Vec<String>) -> String {
    digit_strings
        .iter()
        .map(|s| *codes.get(s).unwrap())
        .collect()
}

fn get_digits_for_line(s: &str) -> Result<String> {
    let mut parts: Vec<Vec<String>> = s
        .split("|")
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().sorted().collect::<String>())
                .collect()
        })
        .collect();

    let left = get_codes(parts.remove(0));
    let digits = parse_to_digits(&left, &parts.remove(0));

    Ok(digits)
}

fn part1(lines: Vec<String>) -> Result<usize> {
    let easy_nums: HashSet<char> = HashSet::from(['1', '4', '7', '8']);
    let result = lines
        .iter()
        .flat_map(|s| s.chars())
        .filter(|digit| easy_nums.contains(digit))
        .count();
    Ok(result)
}

fn part2(lines: Vec<String>) -> Result<usize> {
    let result = lines
        .iter()
        .map(|s| s.parse().unwrap())
        .reduce(|a: usize, b: usize| a + b)
        .unwrap();

    Ok(result)
}
