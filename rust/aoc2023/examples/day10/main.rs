use std::collections::{hash_map::Entry, HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MapValue {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Starting,
}

fn parse_input(input: &str) -> HashMap<(i64, i64), MapValue> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let x = x as i64;
            let y = y as i64;
            match c {
                '|' => {
                    map.insert((x, y), MapValue::Vertical);
                }
                '-' => {
                    map.insert((x, y), MapValue::Horizontal);
                }
                'L' => {
                    map.insert((x, y), MapValue::NorthEast);
                }
                'J' => {
                    map.insert((x, y), MapValue::NorthWest);
                }
                '7' => {
                    map.insert((x, y), MapValue::SouthWest);
                }
                'F' => {
                    map.insert((x, y), MapValue::SouthEast);
                }
                'S' => {
                    map.insert((x, y), MapValue::Starting);
                }
                '.' => {}
                _ => {
                    panic!("unexpected char: {}", c);
                }
            }
        }
    }
    map
}

fn find_paths(game_map: &HashMap<(i64, i64), MapValue>) -> Vec<(i32, Vec<(i64, i64)>)> {
    let &start_pos = game_map
        .iter()
        .find(|(_, v)| **v == MapValue::Starting)
        .unwrap()
        .0;

    let start_pos_neighbors = HashSet::from([
        (start_pos.0 - 1, start_pos.1),
        (start_pos.0 + 1, start_pos.1),
        (start_pos.0, start_pos.1 - 1),
        (start_pos.0, start_pos.1 + 1),
    ]);

    let mut paths = vec![];
    for &path_start_pos in &start_pos_neighbors {
        let Some(_) = game_map.get(&path_start_pos) else {
            // This neighbour isn't a valid path start
            continue;
        };

        let mut steps = 0;
        let mut path = vec![start_pos];

        let mut pos = path_start_pos;

        loop {
            let Some(m) = game_map.get(&pos) else {
                panic!("Pos {:?} not in map!", pos);
            };

            path.push(pos);
            steps += 1;

            if pos == start_pos {
                break;
            }

            let next_pos_options = match m {
                MapValue::Vertical => [(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
                MapValue::Horizontal => [(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
                MapValue::NorthEast => [(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)],
                MapValue::NorthWest => [(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)],
                MapValue::SouthWest => [(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)],
                MapValue::SouthEast => [(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)],
                MapValue::Starting => break,
            };

            let prev_pos = path[path.len() - 2];
            let next_pos = next_pos_options
                .into_iter()
                .find(|&new_pos| prev_pos != new_pos)
                .unwrap();

            pos = next_pos;
        }
        paths.push((steps, path));
    }
    paths
}

fn part1(input: &str) -> Result<String> {
    let game_map = parse_input(input);

    let paths = find_paths(&game_map);

    Ok(paths
        .iter()
        .map(|(steps, _)| steps / 2)
        .max()
        .unwrap()
        .to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DoubleMapEntry {
    Loop,
    Seen,
    Unseen,
}

fn part2(input: &str) -> Result<String> {
    let game_map = parse_input(input);

    let paths = find_paths(&game_map);
    let path = paths.into_iter().max_by_key(|(steps, _)| *steps).unwrap().1;

    let &start_pos = game_map
        .iter()
        .find(|(_, v)| **v == MapValue::Starting)
        .unwrap()
        .0;

    let path_with_start = std::iter::once(start_pos).chain(path.iter().copied());

    let mut double_map = HashMap::new();

    for (i, p) in path_with_start.tuple_windows::<((i64, i64), (i64, i64))>() {
        let y_range = (2 * i.1.min(p.1))..=(2 * i.1.max(p.1));
        let x_range = (2 * i.0.min(p.0))..=(2 * i.0.max(p.0));

        for y in y_range {
            for x in x_range.clone() {
                double_map.insert((x, y), DoubleMapEntry::Loop);
            }
        }
    }

    let mut to_visit = HashSet::new();

    let max_pos = game_map
        .keys()
        .copied()
        .reduce(|a, b| (a.0.max(b.0), a.1.max(b.1)))
        .unwrap();

    for y in -1..(2 * (max_pos.1 + 2)) {
        for x in -1..(2 * (max_pos.0 + 2)) {
            if let Entry::Vacant(e) = double_map.entry((x, y)) {
                e.insert(DoubleMapEntry::Unseen);
                to_visit.insert((x, y));
            }
        }
    }

    let mut sections = vec![];
    while let Some(pos) = to_visit.iter().next().copied() {
        to_visit.remove(&pos);
        let mut found_from_pos = vec![pos];
        let mut visit_from_pos = vec![pos];
        double_map.insert(pos, DoubleMapEntry::Seen);
        while let Some(pos) = visit_from_pos.pop() {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let neighbor = (pos.0 + dx, pos.1 + dy);
                    if let Some(DoubleMapEntry::Unseen) = double_map.get(&neighbor) {
                        found_from_pos.push(neighbor);
                        visit_from_pos.push(neighbor);
                        to_visit.remove(&neighbor);
                        double_map.insert(neighbor, DoubleMapEntry::Seen);
                    }
                }
            }
        }
        sections.push(found_from_pos);
    }

    let mut section_sizes = vec![];
    for section in &sections {
        let section_dedoubled = section
            .iter()
            .filter(|pos| pos.0 % 2 == 0 && pos.1 % 2 == 0)
            .count();
        section_sizes.push(section_dedoubled);
    }

    section_sizes.sort();
    section_sizes.pop();
    let total_size: usize = section_sizes.iter().sum();

    Ok(total_size.to_string())
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let part1_result = match part1(input) {
        Err(ref err) if err.is::<Unimplemented>() => "unimplemented".to_string(),
        result => result?,
    };
    println!("part1: {}", part1_result);
    let part2_result = match part2(input) {
        Err(ref err) if err.is::<Unimplemented>() => "unimplemented".to_string(),
        result => result?,
    };
    println!("part2: {}", part2_result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "4";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);

        let file_data = include_str!("example_input2.txt");
        let expected = "8";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input3.txt");
        let expected = "4";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);

        let file_data = include_str!("example_input4.txt");
        let expected = "8";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
