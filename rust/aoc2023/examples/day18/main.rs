use std::collections::{BTreeMap, BTreeSet};

use anyhow::{bail, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, i32 as nom_i32, line_ending},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    num: i32,
    color: String,
}

fn instruction_from_color(color: &str) -> Result<Instruction> {
    // 6 digit color -> 5 hex digit, 1 number from 0 to 3.
    let (num, dir_num) = color.split_at(5);

    let dir = match dir_num {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => bail!("invalid color: {}", color),
    };
    let num = i32::from_str_radix(num, 16)?;

    Ok(Instruction {
        dir,
        num,
        color: color.to_string(),
    })
}

#[derive(Debug)]
struct Instructions {
    instructions: Vec<Instruction>,
}

fn parse_dir(input: &str) -> IResult<&str, Direction> {
    let (input, c) = alt((tag("U"), tag("R"), tag("D"), tag("L")))(input)?;
    let dir = match c {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        _ => unreachable!(),
    };
    Ok((input, dir))
}

fn parse_color(input: &str) -> IResult<&str, String> {
    let (input, color) = delimited(tag("(#"), alphanumeric1, tag(")"))(input)?;
    Ok((input, color.to_string()))
}

fn parse_nom(input: &str) -> IResult<&str, Instructions> {
    separated_list1(
        line_ending,
        tuple((
            terminated(parse_dir, tag(" ")),
            terminated(nom_i32, tag(" ")),
            parse_color,
        )),
    )(input)
    .map(|(input, instructions)| {
        (
            input,
            Instructions {
                instructions: instructions
                    .into_iter()
                    .map(|(dir, num, color)| Instruction { dir, num, color })
                    .collect(),
            },
        )
    })
}

fn parse(input: &'static str) -> Result<Instructions> {
    let (_, instructions) = parse_nom(input).map_err(|err| anyhow::anyhow!(err))?;
    Ok(instructions)
}

fn is_outside(
    grid: &BTreeMap<(i32, i32), String>,
    extents: (i32, i32, i32, i32),
    pos: (i32, i32),
    known_outside: &mut BTreeSet<(i32, i32)>,
    known_inside: &mut BTreeSet<(i32, i32)>,
) -> bool {
    if known_outside.contains(&pos) {
        return true;
    }
    if known_inside.contains(&pos) {
        return false;
    }
    let mut outside = false;
    let mut to_visit = vec![pos];
    let mut visited = BTreeSet::new();

    while let Some(pos) = to_visit.pop() {
        if !visited.insert(pos) {
            continue;
        }

        if known_outside.contains(&pos) {
            outside = true;
            continue;
        }

        for d in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            let (dx, dy) = d.delta();
            let new_pos = (pos.0 + dx, pos.1 + dy);
            if new_pos.0 < extents.0
                || new_pos.0 > extents.1
                || new_pos.1 < extents.2
                || new_pos.1 > extents.3
            {
                outside = true;
                continue;
            }
            if grid.get(&new_pos).is_some() {
                continue;
            }

            to_visit.push(new_pos);
        }
    }

    if outside {
        for pos in visited {
            known_outside.insert(pos);
        }
    } else {
        for pos in visited {
            known_inside.insert(pos);
        }
    }

    outside
}

fn part1(input: &'static str) -> Result<String> {
    let instructions = parse(input)?;
    let grid = construct_grid(instructions);

    let (min_x, max_x, min_y, max_y) = grid.keys().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (
                min_x.min(*x - 1),
                max_x.max(*x + 1),
                min_y.min(*y - 1),
                max_y.max(*y + 1),
            )
        },
    );
    let extents = (min_x, max_x, min_y, max_y);

    println!("({},{}) -> ({},{})", min_x, min_y, max_x, max_y);

    // let mut debug_hashmap = BTreeMap::new();

    let mut known_outside = BTreeSet::new();
    for x in min_x..=max_x {
        known_outside.insert((x, min_y));
        known_outside.insert((x, max_y));
    }
    for y in min_y..=max_y {
        known_outside.insert((min_x, y));
        known_outside.insert((max_x, y));
    }
    let mut known_inside = BTreeSet::new();
    let mut total = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // println!("({},{})", x, y);
            if grid.get(&(x, y)).is_some() {
                total += 1;
                print!("#");
                continue;
            }
            if is_outside(
                &grid,
                extents,
                (x, y),
                &mut known_outside,
                &mut known_inside,
            ) {
                print!("O");
            } else {
                print!("I");
                total += 1;
            }
        }
        println!();
    }
    Ok(total.to_string())
}

fn construct_grid(instructions: Instructions) -> BTreeMap<(i32, i32), String> {
    let mut grid = BTreeMap::new();
    let mut pos = (0, 0);
    grid.insert(pos, "white".to_string());
    for instruction in instructions.instructions {
        let (dx, dy) = instruction.dir.delta();
        for _ in 0..instruction.num {
            let new_pos = (pos.0 + dx, pos.1 + dy);
            let color = instruction.color.clone();
            grid.insert(new_pos, color);
            pos = new_pos;
        }
    }
    grid
}
fn part2(input: &'static str) -> Result<String> {
    let instructions = parse(input)?;
    let instructions = instructions
        .instructions
        .iter()
        .map(|i| instruction_from_color(&i.color))
        .collect::<Result<Vec<_>>>()?;

    let mut points = vec![(0, 0)];

    let mut pos = (0i64, 0i64);
    let mut b = 0i64;
    for instruction in instructions {
        let (dx, dy) = instruction.dir.delta();
        let (dx, dy) = (dx as i64, dy as i64);
        let new_pos = (
            pos.0 + dx * instruction.num as i64,
            pos.1 + dy * instruction.num as i64,
        );
        b += instruction.num as i64;
        points.push(new_pos);
        pos = new_pos;
    }
    let mut total = 0i64;
    for i in 0..points.len() {
        let (x0, _y0) = points[i];
        let (_x1, y1) = points[(i + 1) % points.len()];
        let (x2, _y2) = points[(i + 2) % points.len()];
        let area = y1 * (x0 - x2);
        total += area;
    }

    total = total.abs() / 2;

    let i = total - b / 2 + 1;

    Ok((i + b).to_string())
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
        let expected = "62";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        // let expected = "62";
        let expected = "952408144115";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
