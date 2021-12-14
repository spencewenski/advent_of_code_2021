use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;
use std::iter::FromIterator;

pub fn day14(args: &Arguments) -> anyhow::Result<()> {
    let mut reader = reader(args.src_file.as_ref())?;

    let mut poly_template = String::new();
    reader.read_line(&mut poly_template)?;
    poly_template = poly_template.trim().to_owned();

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        lines.push(line);
    }
    let pairs = parse_insertion_pairs(lines);

    let result = if args.part == 1 {
        build_polymer(poly_template, pairs, 10)
    } else {
        build_polymer(poly_template, pairs, 40)
    }?;

    info!("{:?}", result);

    Ok(())
}

fn parse_insertion_pairs(lines: Vec<String>) -> HashMap<String, char> {
    let mut pairs = HashMap::new();

    for line in lines {
        let mut parts = line
            .split("->")
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>();

        pairs.insert(parts.remove(0), parts.remove(0).chars().nth(0).unwrap());
    }

    pairs
}

fn step(
    pair_counts: HashMap<String, u64>,
    char_counts: HashMap<char, u64>,
    pairs: &HashMap<String, char>,
) -> (HashMap<String, u64>, HashMap<char, u64>) {
    let mut new_pair_counts = pair_counts.clone();
    let mut new_char_counts = char_counts.clone();

    for entry in pair_counts {
        let left = entry.0.chars().nth(0).unwrap();
        let right = entry.0.chars().nth(1).unwrap();

        let middle = pairs.get(&entry.0).unwrap();

        // Decrement old pair
        let count = new_pair_counts.entry(entry.0).or_insert(0);
        *count -= entry.1;

        // Increment new pairs
        let count = new_pair_counts
            .entry(format!("{}{}", left, middle))
            .or_insert(0);
        *count += entry.1;

        let count = new_pair_counts
            .entry(format!("{}{}", middle, right))
            .or_insert(0);
        *count += entry.1;

        // Increment the character counts
        let count = new_char_counts.entry(*middle).or_insert(0);
        *count += entry.1;
    }

    (new_pair_counts, new_char_counts)
}

fn build_polymer(
    poly_template: String,
    pairs: HashMap<String, char>,
    num_steps: usize,
) -> Result<u64> {
    // Init the pair counts map
    let pair_counts =
        poly_template
            .chars()
            .collect_vec()
            .windows(2)
            .fold(HashMap::new(), |mut accum, window| {
                let count = accum.entry(String::from_iter(window)).or_insert(0);
                *count += 1;
                accum
            });

    // Init the character counts map
    let char_counts = poly_template.chars().fold(HashMap::new(), |mut accum, c| {
        let count = accum.entry(c).or_insert(0);
        *count += 1;
        accum
    });

    let mut result = (pair_counts, char_counts);
    for _ in 0..num_steps {
        result = step(result.0, result.1, &pairs);
    }

    let max = result.1.values().max().unwrap();
    let min = result.1.values().min().unwrap();

    Ok(max - min)
}
