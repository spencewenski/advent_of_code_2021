use crate::arguments::Arguments;
use crate::io::reader;
use std::io::BufRead;

pub fn day14(args: &Arguments) -> anyhow::Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
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

fn part1(lines: Vec<String>) -> anyhow::Result<u64> {
    Ok(0)
}

fn part2(lines: Vec<String>) -> anyhow::Result<u64> {
    Ok(0)
}
