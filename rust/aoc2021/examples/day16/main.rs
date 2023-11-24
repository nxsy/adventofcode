// cargo run --example day16 -- (part1|part2) (example_input|final_input)

use std::fs::read_to_string;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day16", about = "Advent of Code 2021 Day 9")]
pub struct Args {
    part: Part,
    input: Input,
}

enum TypeId {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    Greater = 5,
    Less = 6,
    Equal = 7,
}

impl From<u32> for TypeId {
    fn from(v: u32) -> Self {
        match v {
            0 => TypeId::Sum,
            1 => TypeId::Product,
            2 => TypeId::Minimum,
            3 => TypeId::Maximum,
            4 => TypeId::Literal,
            5 => TypeId::Greater,
            6 => TypeId::Less,
            7 => TypeId::Equal,
            _ => panic!("Unknown packet type id"),
        }
    }
}

struct Parser {
    chars: Vec<char>,
    position: usize,
}

impl Parser {
    fn new(s: String) -> Self {
        Self {
            chars: s.chars().collect(),
            position: 0,
        }
    }

    fn string(&mut self, bit_length: usize) -> String {
        let res = self.chars[self.position..self.position + bit_length]
            .iter()
            .collect::<String>();
        self.position += bit_length;
        res
    }

    fn u32(&mut self, bit_length: usize) -> u32 {
        u32::from_str_radix(&self.string(bit_length), 2).unwrap()
    }

    fn bool(&mut self) -> bool {
        self.u32(1) == 1
    }

    fn trim(&mut self) {
        self.position += (8 - self.position % 8) % 8;
    }

    fn literal(&mut self) -> u64 {
        let mut literal_bits = String::new();
        loop {
            let progress = self.bool();
            literal_bits.push_str(&self.string(4));
            if !progress {
                break;
            }
        }
        u64::from_str_radix(&literal_bits, 2).unwrap()
    }

    fn is_empty(&self) -> bool {
        self.position == self.chars.len()
    }

    fn packet(&mut self) -> PacketMetadata {
        let start_position = self.position;
        let version = self.u32(3);
        let mut summed_version = version;
        let packet_id = TypeId::from(self.u32(3));

        let (packet, result) = if let TypeId::Literal = packet_id {
            let value = self.literal();
            (Packet::Literal { value }, value)
        } else {
            let length_type_id = self.bool();
            let mut subpackets = Vec::new();
            if !length_type_id {
                let subpackets_length = self.u32(15) as usize;
                let target_position = self.position + subpackets_length;
                while self.position != target_position {
                    let subpacket = self.packet();
                    summed_version += subpacket.summed_version;
                    subpackets.push(subpacket);
                }
            } else {
                let num_subpackets = self.u32(11);
                for _ in 0..num_subpackets {
                    let subpacket = self.packet();
                    summed_version += subpacket.summed_version;
                    subpackets.push(subpacket);
                }
            }
            let iter = subpackets.iter().map(|p| p.result);

            let result: u64 = match packet_id {
                TypeId::Sum => Some(iter.sum::<u64>()),
                TypeId::Product => iter.reduce(|x, y| x * y),
                TypeId::Minimum => iter.reduce(|x, y| x.min(y)),
                TypeId::Maximum => iter.reduce(|x, y| x.max(y)),
                TypeId::Greater => iter.reduce(|x, y| (x > y) as u64),
                TypeId::Less => iter.reduce(|x, y| (x < y) as u64),
                TypeId::Equal => iter.reduce(|x, y| (x == y) as u64),
                TypeId::Literal => panic!(),
            }
            .unwrap();
            (Packet::Operator { subpackets }, result)
        };
        let packet_length = self.position - start_position;

        PacketMetadata {
            version,
            summed_version,
            packet,
            packet_length,
            result,
        }
    }
}

#[derive(Debug)]
enum Packet {
    Literal { value: u64 },
    Operator { subpackets: Vec<PacketMetadata> },
}

#[derive(Debug)]
struct PacketMetadata {
    version: u32,
    summed_version: u32,
    packet_length: usize,
    result: u64,
    packet: Packet,
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let chars: Vec<_> = contents
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_string())
        .collect();

    let mut bits = String::new();
    for char_pair in chars.chunks(2) {
        let s = String::new() + &char_pair[0] + &char_pair[1];
        let b = u8::from_str_radix(&s, 16).unwrap();
        bits.push_str(&format!("{:08b}", &b));
    }
    let mut p = Parser::new(bits);

    let mut total_summed_version = 0;
    let mut results = Vec::new();
    while !p.is_empty() {
        let packet = p.packet();
        println!("Summed version: {}", packet.summed_version);
        total_summed_version += packet.summed_version;
        results.push(packet.result);
        p.trim();
    }

    println!("Total: {}", total_summed_version);
    println!("Results: {:?}", results);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day16/example_input",
        Input::FinalInput => "data/day16/input",
    };

    solve(file_path)
}
