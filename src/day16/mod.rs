use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use bitvec::prelude::*;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::BufRead;

pub fn day16(args: &Arguments) -> Result<()> {
    let mut reader = reader(args.src_file.as_ref())?;
    let mut chars = String::new();
    reader.read_line(&mut chars)?;

    let chars = chars.trim().chars().collect_vec();

    let result = if args.part == 1 {
        part1(chars)
    } else {
        part2(chars)
    }?;

    info!("{:?}", result);

    Ok(())
}

const ONE_BIT_MASK: u64 = 0b1;
const THREE_BIT_MASK: u64 = 0b111;
const FOUR_BIT_MASK: u64 = 0b1111;

fn hex_to_binary(hex: char) -> u64 {
    match hex {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => unreachable!("This should be unreachable!"),
    }
}

#[derive(Debug)]
enum LengthTypeId {
    Bits,
    Packets,
}

impl LengthTypeId {
    fn from_bit(bit: u64) -> LengthTypeId {
        match bit {
            0 => LengthTypeId::Bits,
            1 => LengthTypeId::Packets,
            _ => unreachable!("Illegal bit for Length Type Id!"),
        }
    }
}

#[derive(Debug)]
enum TypeId {
    Literal,
    Operator,
}

impl TypeId {
    fn from_number(num: u64) -> TypeId {
        match num {
            4 => TypeId::Literal,
            0 | 1 | 2 | 3 | 5 | 6 | 7 => TypeId::Operator,
            _ => unreachable!("Illegal bits for Type Id"),
        }
    }
}

#[derive(Debug)]
struct Header {
    version: u64,
    type_id: TypeId,
}

fn number_from_bits<T1, T2>(bits: &BitVec<T1, T2>, start: usize, num_bits: usize) -> u64
where
    T1: bitvec::order::BitOrder,
    T2: bitvec::prelude::BitStore,
{
    let mut result = 0;
    for i in start..num_bits {
        result = result << 1;
        result = result | bool_to_bit(bits[i])
    }
    result
}

impl Header {
    fn from_bits<T1, T2>(bits: &BitVec<T1, T2>, start: usize) -> Header
    where
        T1: bitvec::order::BitOrder,
        T2: bitvec::prelude::BitStore,
    {
        let version = number_from_bits(bits, start, start + 3);
        let type_id = number_from_bits(bits, start + 3, start + 6);
        let type_id = TypeId::from_number(type_id);
        Header { version, type_id }
    }
}

#[derive(Debug)]
struct Packet {
    header: Header,
}

fn bit_to_bool(b: u64) -> bool {
    match b {
        0 => false,
        1 => true,
        _ => unreachable!("Invalid bit to convert to bool: {}", b),
    }
}

fn bool_to_bit(b: bool) -> u64 {
    match b {
        true => 1,
        false => 0,
    }
}

fn hex_to_bitvec(chars: &[char]) -> BitVec<Msb0, u64> {
    chars
        .into_iter()
        .map(|c| hex_to_binary(*c))
        .fold(bitvec![Msb0, u64;], |mut accum, b| {
            for i in 0..4 {
                let val = (b >> (4 - 1 - i)) & ONE_BIT_MASK;
                let val = bit_to_bool(val);
                accum.push(val);
            }
            accum
        })
}

fn read_packet<T1, T2>(bits: &BitVec<T1, T2>, start: usize) -> Packet
where
    T1: bitvec::order::BitOrder,
    T2: bitvec::prelude::BitStore,
{
    let header = Header::from_bits(bits, start);

    Packet { header }
}

fn part1(chars: Vec<char>) -> Result<usize> {
    let bits: BitVec<Msb0, u64> = hex_to_bitvec(&chars);

    let packet = read_packet(&bits, 0);
    info!("{:?}", packet);

    Ok(0)
}

fn part2(chars: Vec<char>) -> Result<usize> {
    Ok(0)
}

fn print_bitvec<T1, T2>(bits: &BitVec<T1, T2>)
where
    T1: bitvec::order::BitOrder,
    T2: bitvec::prelude::BitStore,
{
    for b in bits {
        print!("{}", if b == true { 1 } else { 0 })
    }
    println!();
}
