use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::io::BufRead;

pub fn day3(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // let line = line.parse()?;
        let line = line.chars().into_iter().collect();
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
struct BitCount {
    zero: i64,
    one: i64,
}

fn count_bits(lines: &Vec<Vec<char>>) -> Result<Vec<BitCount>> {
    let size = if let Some(v) = lines.first() {
        v.len()
    } else {
        0
    };

    if size <= 0 {
        return Err(anyhow::Error::msg("Invalid input!"));
    }

    let mut totals = Vec::new();
    for _ in 0..size {
        totals.push(BitCount::default());
    }

    for line in lines {
        for i in 0..line.len() {
            let c = line[i];
            match c {
                '0' => totals[i].zero += 1,
                '1' => totals[i].one += 1,
                _ => return Err(anyhow::Error::msg("Invalid input!")),
            }
        }
    }

    Ok(totals)
}

fn part1(lines: Vec<Vec<char>>) -> Result<i64> {
    let totals = count_bits(&lines)?;

    info!("{:?}", totals);

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..totals.len() {
        let count = &totals[i];
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if count.one > count.zero {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    info!("gamma: {}, epsilon: {}", gamma, epsilon);

    Ok(gamma * epsilon)
}

fn part2(lines: Vec<Vec<char>>) -> Result<i64> {
    let totals = count_bits(&lines)?;

    let mut oxygen = lines.clone();
    let mut co2 = lines.clone();

    for i in 0..totals.len() {
        let oxygen_totals = count_bits(&oxygen)?;
        let co2_totals = count_bits(&co2)?;

        info!("num oxygen: {}, num co2: {}", oxygen.len(), co2.len());

        let most_common = if oxygen_totals[i].one >= oxygen_totals[i].zero {
            '1'
        } else {
            '0'
        };
        let least_common = if co2_totals[i].zero <= co2_totals[i].one {
            '0'
        } else {
            '1'
        };

        if oxygen.len() > 1 {
            oxygen = oxygen.into_iter().filter(|v| v[i] == most_common).collect();
        }
        if co2.len() > 1 {
            co2 = co2.into_iter().filter(|v| v[i] == least_common).collect();
        }
    }

    if oxygen.len() != 1 || co2.len() != 1 {
        return Err(anyhow::Error::msg("Invalid input!"));
    }

    let oxygen = chars_to_decimal(oxygen.first().ok_or(anyhow::Error::msg("Invalid input!"))?)?;
    let co2 = chars_to_decimal(co2.first().ok_or(anyhow::Error::msg("Invalid input!"))?)?;

    Ok(oxygen * co2)
}

fn chars_to_decimal(chars: &Vec<char>) -> Result<i64> {
    let mut result = 0;

    for c in chars {
        result = result << 1;
        if *c == '1' {
            result |= 1;
        }
    }

    Ok(result)
}
