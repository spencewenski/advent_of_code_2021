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
            .map(|c| String::from(c).parse().unwrap())
            .collect::<Vec<u8>>();
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

fn part1(lines: Vec<Vec<u8>>) -> Result<u64> {
    Ok(0)
}

fn part2(lines: Vec<Vec<u8>>) -> Result<u64> {
    Ok(0)
}
