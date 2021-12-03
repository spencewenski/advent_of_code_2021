use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::io::BufRead;

pub fn day3(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines: Vec<i64> = Vec::new();
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

fn part1(numbers: Vec<i64>) -> Result<()> {
    Ok(())
}

fn part2(numbers: Vec<i64>) -> Result<()> {
    Ok(())
}
