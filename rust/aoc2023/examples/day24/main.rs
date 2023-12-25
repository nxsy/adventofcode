use std::{fs::File, io::Write, ops::RangeInclusive};

use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;

mod parsing;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Pos {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hailstone {
    pos: Pos,
    dir: Pos,
}

impl Hailstone {
    fn intersect(&self, other: &Hailstone) -> Option<Pos> {
        let det = other.dir.x * self.dir.y - other.dir.y * self.dir.x;
        if det == 0.0 {
            return None;
        }

        let dx = other.pos.x - self.pos.x;
        let dy = other.pos.y - self.pos.y;

        let u = (dy * other.dir.x - dx * other.dir.y) / det;
        let v = (dy * self.dir.x - dx * self.dir.y) / det;

        (u >= 0.0 && v >= 0.0).then_some(Pos {
            x: self.pos.x + self.dir.x * u,
            y: self.pos.y + self.dir.y * u,
            z: self.pos.z + self.dir.z * u,
        })
    }
}

#[derive(Debug)]
struct ParsedInput {
    hailstones: Vec<Hailstone>,
}

fn part1(input: &'static str, r: RangeInclusive<f64>) -> Result<String> {
    let input = parsing::parse(input)?;

    let mut total = 0;
    for (hs1, hs2) in input.hailstones.iter().tuple_combinations() {
        if let Some(pos) = hs1.intersect(hs2) {
            if r.contains(&pos.x) && r.contains(&pos.y) {
                total += 1;
            }
        }
    }
    Ok(total.to_string())
}

fn part2_use_python_solver(input: &'static str) -> Result<String> {
    let input = parsing::parse(input)?;

    let mut file = File::create("solver.py")?;

    writeln!(
        file,
        "#!/usr/bin/env python3

from z3 import *

px, py, pz = Int('px'), Int('py'), Int('pz')
dx, dy, dz = Int('dx'), Int('dy'), Int('dz')

solver = Solver()"
    )?;

    for (i, h) in input.hailstones.iter().enumerate() {
        writeln!(file, "\nt{i} = Int('t{i}')")?;
        writeln!(
            file,
            "solver.add(
    And(
        px + dx * t{i} == {px} + {dx} * t{i},
        py + dy * t{i} == {py} + {dy} * t{i},
        pz + dz * t{i} == {pz} + {dz} * t{i},
    )
)",
            px = h.pos.x,
            dx = h.dir.x,
            py = h.pos.y,
            dy = h.dir.y,
            pz = h.pos.z,
            dz = h.dir.z,
        )?;
    }

    writeln!(
        file,
        "\nsolver.check()
model = solver.model()
print(model.eval(px + py + pz))",
    )?;

    Ok("solver.py".to_string())
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let part1_result = match part1(input, 200000000000000.0..=400000000000000.0) {
        Err(ref err) if err.is::<Unimplemented>() => "unimplemented".to_string(),
        result => result?,
    };
    println!("part1: {}", part1_result);
    let part2_result = match part2_use_python_solver(input) {
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
        let expected = "2";
        let actual = part1(file_data, 7.0..=27.0)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
