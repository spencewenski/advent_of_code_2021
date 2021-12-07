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
        simulate_fish(numbers, 80)
    } else {
        simulate_fish(numbers, 256)
    }?;

    info!("{:?}", result);

    Ok(())
}

const RESET_COUNT: usize = 6;
const INITIAL_COUNT: usize = 8;

fn simulate_day(counts: &mut Vec<u64>) {
    let num_at_zero = counts.remove(0);
    counts.push(num_at_zero);
    counts[RESET_COUNT] += num_at_zero;
}

fn simulate_fish(numbers: Vec<usize>, num_days: usize) -> Result<u64> {
    let mut counts: Vec<u64> = Vec::new();

    for _ in 0..INITIAL_COUNT + 1 {
        counts.push(0);
    }

    for num in numbers {
        counts[num] += 1;
    }

    for _ in 0..num_days {
        simulate_day(&mut counts);
    }

    let total = counts.into_iter().reduce(|a, b| a + b).unwrap();

    Ok(total)
}
