// cargo run --example day19 -- (part1|part2) (example_input|final_input)

use std::collections::HashMap;
use std::collections::HashSet;
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
#[structopt(name = "day19", about = "Advent of Code 2021 Day 19")]
pub struct Args {
    part: Part,
    input: Input,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(i32, i32, i32);

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Point {
    fn rotate(&self, rot: usize) -> Point {
        match rot {
            0 => Point(self.0, self.1, self.2),
            1 => Point(self.0, self.2, -self.1),
            2 => Point(self.0, -self.1, -self.2),
            3 => Point(self.0, -self.2, self.1),
            4 => Point(self.1, self.0, -self.2),
            5 => Point(self.1, self.2, self.0),
            6 => Point(self.1, -self.0, self.2),
            7 => Point(self.1, -self.2, -self.0),
            8 => Point(self.2, self.0, self.1),
            9 => Point(self.2, self.1, -self.0),
            10 => Point(self.2, -self.0, -self.1),
            11 => Point(self.2, -self.1, self.0),
            12 => Point(-self.0, self.1, -self.2),
            13 => Point(-self.0, self.2, self.1),
            14 => Point(-self.0, -self.1, self.2),
            15 => Point(-self.0, -self.2, -self.1),
            16 => Point(-self.1, self.0, self.2),
            17 => Point(-self.1, self.2, -self.0),
            18 => Point(-self.1, -self.0, -self.2),
            19 => Point(-self.1, -self.2, self.0),
            20 => Point(-self.2, self.0, -self.1),
            21 => Point(-self.2, self.1, self.0),
            22 => Point(-self.2, -self.0, self.1),
            23 => Point(-self.2, -self.1, -self.0),
            _ => panic!(),
        }
    }
}

#[derive(Default, Clone)]
struct ScannerData {
    correct_transform: Option<(usize, Point)>,
    rotated_positions: HashMap<usize, Vec<Point>>,
}

impl ScannerData {
    fn add_position(&mut self, pos: Point) {
        for rot in 0..24 {
            self.rotated_positions
                .entry(rot)
                .or_default()
                .push(pos.rotate(rot));
        }
    }
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let mut lines = contents.lines();
    let mut scanner_data = Vec::new();
    let mut scanner = ScannerData::default();
    lines.next();
    for line in lines {
        if line.starts_with("--- scanner ") {
            continue;
        }
        if line.is_empty() {
            scanner_data.push(scanner);
            scanner = ScannerData::default();
            continue;
        }
        let pos: Vec<_> = line.split(',').collect();
        let pos = Point(
            pos[0].parse::<i32>().unwrap(),
            pos[1].parse::<i32>().unwrap(),
            pos[2].parse::<i32>().unwrap(),
        );
        scanner.add_position(pos);
    }
    scanner_data.push(scanner);

    scanner_data[0].correct_transform = Some((0, Point(0, 0, 0)));
    let mut known_scanners = HashSet::from([0]);
    let mut known_points = HashSet::new();

    loop {
        for s in scanner_data.iter() {
            let (rot, translation) = match s.correct_transform {
                Some(v) => v,
                None => continue,
            };
            for &p in &s.rotated_positions[&rot] {
                known_points.insert(translation + p);
            }
        }

        if known_scanners.len() == scanner_data.len() {
            break;
        }

        let mut found = false;
        'new_scanners: for (j, s2) in scanner_data.iter_mut().enumerate() {
            if s2.correct_transform.is_some() {
                continue;
            }

            for rot in 0..24 {
                let mut translations = HashMap::new();

                for &p1 in &known_points {
                    for &p2 in &s2.rotated_positions[&rot] {
                        let translation = p1 - p2;
                        translations
                            .entry(translation)
                            .and_modify(|x| *x += 1)
                            .or_insert(1);
                        if translations[&translation] > 11 {
                            s2.correct_transform = Some((rot, translation));
                            known_scanners.insert(j);
                            found = true;
                            continue 'new_scanners;
                        }
                    }
                }
            }
        }
        if !found {
            panic!("No progress!");
        }
    }
    println!("Known points: {}", known_points.len());

    let scanner_locations: Vec<_> = scanner_data
        .iter()
        .map(|s| s.correct_transform.unwrap().1)
        .collect();
    let mut highest_manhattan: Option<i32> = None;
    for (i, &loc1) in scanner_locations.iter().enumerate() {
        for &loc2 in scanner_locations.iter().skip(i + 1) {
            let vector = loc1 - loc2;
            let manhattan = vector.0.abs() + vector.1.abs() + vector.2.abs();
            match highest_manhattan {
                Some(v) => highest_manhattan = Some(v.max(manhattan)),
                None => highest_manhattan = Some(manhattan),
            }
        }
    }
    println!("Highest Manhattan distance: {}", highest_manhattan.unwrap());

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day19/example_input",
        Input::FinalInput => "data/day19/input",
    };

    solve(file_path)
}
