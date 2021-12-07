use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::cmp::{max, min};
use std::io::BufRead;

pub fn day7(args: &Arguments) -> Result<()> {
    let mut reader = reader(args.src_file.as_ref())?;

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let numbers = line.split(",").map(|s| s.trim().parse().unwrap()).collect();

    let result = if args.part == 1 {
        min_fuel(numbers, |a, b| max(a, b) - min(a, b))
    } else {
        min_fuel(numbers, |a, b| {
            let distance = max(a, b) - min(a, b);
            (distance * (distance + 1)) / 2
        })
    }?;

    info!("{:?}", result);

    Ok(())
}

fn min_fuel<F>(numbers: Vec<u64>, fuel_calculator: F) -> Result<u64>
where
    F: Fn(u64, u64) -> u64,
{
    let min_position = *numbers.iter().min().unwrap();
    let max_position = *numbers.iter().max().unwrap();

    let result = (min_position..=max_position)
        .map(|to| numbers.iter().map(|from| fuel_calculator(*from, to)).sum())
        .min()
        .unwrap();
    Ok(result)
}
