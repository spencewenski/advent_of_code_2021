use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::io::BufRead;

pub fn day6(args: &Arguments) -> Result<()> {
    let mut reader = reader(args.src_file.as_ref())?;

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let numbers = line.split(",").map(|s| s.trim().parse().unwrap()).collect();

    let result = if args.part == 1 {
        part1(&numbers)
    } else {
        part2(&numbers)
    }?;

    info!("{:?}", result);

    Ok(())
}

fn part1(numbers: &Vec<u32>) -> Result<usize> {
    Ok(0)
}

fn part2(numbers: &Vec<u32>) -> Result<usize> {
    Ok(0)
}
