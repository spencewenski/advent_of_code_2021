use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;

pub fn day8(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines = Vec::new();
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
struct Line {
    left: Vec<String>,
    right: Vec<String>,
}

impl Line {
    fn from_str(s: &str) -> Result<Line> {
        let mut parts: Vec<Vec<String>> = s
            .split("|")
            .map(|s| {
                s.split(" ")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_owned())
                    .collect()
            })
            .collect();

        let line = Line {
            left: parts.remove(0),
            right: parts.remove(0),
        };

        Ok(line)
    }
}

fn part1(lines: Vec<Line>) -> Result<usize> {
    let easy_nums: HashSet<usize> = HashSet::from([2, 3, 4, 7]);

    let result = lines
        .into_iter()
        .flat_map(|line| line.right)
        .filter(|number| easy_nums.contains(&number.len()))
        .count();

    Ok(result)
}

fn part2(lines: Vec<Line>) -> Result<usize> {
    Ok(0)
}
