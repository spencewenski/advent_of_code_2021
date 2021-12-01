use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::io::BufRead;

pub fn day1(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut numbers: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let num = line.parse()?;
        numbers.push(num);
    }
    let numbers = numbers;

    let result = if args.part == 1 {
        count_increasing_windows(&numbers, 2)
    } else {
        count_increasing_windows(&numbers, 4)
    }?;

    info!("{:?}", result);

    Ok(())
}

fn count_increasing_windows(numbers: &Vec<i64>, window_size: usize) -> Result<usize> {
    let count = numbers
        .windows(window_size)
        .filter(|window| window[0] < window[window_size - 1])
        .count();

    Ok(count)
}
