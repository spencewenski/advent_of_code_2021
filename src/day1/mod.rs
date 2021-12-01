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
        part1(&numbers)
    } else {
        part2(&numbers)
    }?;

    info!("{:?}", result);

    Ok(())
}

fn part1(numbers: &Vec<i64>) -> Result<i32> {
    let mut count = 0;
    let mut previous = numbers[0];
    for i in 1..numbers.len() {
        let next = numbers[i];
        if next > previous {
            count += 1;
        }
        previous = next;
    }

    Ok(count)
}

fn part2(numbers: &Vec<i64>) -> Result<i32> {
    let mut count = 0;

    let mut previous_sum = numbers[0] + numbers[1] + numbers[2];

    for i in 0..numbers.len() - 3 {
        let next_sum = previous_sum - numbers[i] + numbers[i + 3];
        if next_sum > previous_sum {
            count += 1;
        }
        previous_sum = next_sum;
    }

    Ok(count)
}
