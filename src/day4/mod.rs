use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::io::BufRead;

type LineType = i64;

pub fn day4(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines: Vec<LineType> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.parse()?;
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

fn part1(lines: Vec<LineType>) -> Result<i64> {
    Ok(0)
}

fn part2(lines: Vec<LineType>) -> Result<i64> {
    Ok(0)
}
