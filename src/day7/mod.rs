use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::io::BufRead;

pub fn day7(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut numbers: Vec<u64> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let num = line.parse()?;
        numbers.push(num);
    }
    let numbers = numbers;

    let result = if args.part == 1 {
        part1(&numbers)
    } else {
        part2(&numbers)
    }?;

    info!("{:?}", result);

    Ok(())
}

fn part1(numbers: &Vec<u64>) -> Result<usize> {
    Ok(0)
}

fn part2(numbers: &Vec<u64>) -> Result<usize> {
    Ok(0)
}
