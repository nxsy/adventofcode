use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use thiserror::Error;

mod parsing;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

type Pos = (i32, i32);

trait Map {
    fn traversable_tiles(&self) -> Vec<Pos>;
    fn traversable_neighbors(&self, pos: Pos, follow_slopes: bool) -> Vec<Pos>;
    fn get_starting_position(&self) -> Pos;
    fn get_target_position(&self) -> Pos;
}

fn part1(input: &str, use_dfs: bool) -> Result<String> {
    let map = parsing::parse(input)?;

    let longest_path = if use_dfs {
        solve_dfs(map, true)
    } else {
        part1_bfs(map)
    };

    Ok(longest_path.to_string())
}

fn part1_bfs<M: Map>(map: M) -> usize {
    let (start_position, target_position) =
        (map.get_starting_position(), map.get_target_position());

    let mut longest_path = 0;
    let mut stack: Vec<(Pos, BTreeSet<Pos>)> = vec![(start_position, BTreeSet::new())];
    while let Some((node, visited)) = stack.pop() {
        if node == target_position {
            longest_path = longest_path.max(visited.len());
            continue;
        }

        if visited.contains(&node) {
            continue;
        }
        let mut visited = visited.clone();
        visited.insert(node);

        for n in map.traversable_neighbors(node, true) {
            stack.push((n, visited.clone()));
        }
    }
    longest_path
}

fn part2(input: &str) -> Result<String> {
    let map = parsing::parse(input)?;

    let longest_path = solve_dfs(map, false);

    Ok(longest_path.to_string())
}

fn solve_dfs<M: Map>(map: M, follow_slopes: bool) -> usize {
    let (start_position, target_position) =
        (map.get_starting_position(), map.get_target_position());

    let mut junctions = BTreeSet::new();
    for pos in map.traversable_tiles() {
        /*

        Always consider slopes traversable to determine junctions, or we
        can't treat passages between junctions to be mutually exclusive in
        terms of tiles traversed:

         X.>A<.Y
            v
            .
            Z

        A should be considered a junction even though it can only traverse
        towards Z.  If not, X and Y would both traverse the path from A
        to Z.

        */

        let neighbors = map.traversable_neighbors(pos, false);
        if neighbors.len() > 2 || pos == start_position || pos == target_position {
            junctions.insert(pos);
        }
    }

    let mut passages: BTreeMap<Pos, Vec<(Pos, usize)>> = BTreeMap::new();
    for v in &junctions {
        let mut stack = vec![];
        let mut visited = BTreeSet::new();

        stack.push((*v, 0));
        while let Some((n, length)) = stack.pop() {
            if visited.contains(&n) {
                continue;
            }
            visited.insert(n);

            if junctions.contains(&n) && n != *v {
                passages.entry(*v).or_default().push((n, length));
                continue;
            }

            for neighbor in map.traversable_neighbors(n, follow_slopes) {
                stack.push((neighbor, length + 1));
            }
        }
    }

    let mut dfs_helper = DfsHelper::new(passages, target_position);
    dfs_helper.dfs(start_position, 0);
    dfs_helper.max_length
}

struct DfsHelper {
    passages: BTreeMap<Pos, Vec<(Pos, usize)>>,
    target_position: Pos,

    seen: BTreeMap<Pos, bool>,
    max_length: usize,
}

impl DfsHelper {
    fn new(passages: BTreeMap<Pos, Vec<(Pos, usize)>>, target_position: Pos) -> Self {
        let mut seen = BTreeMap::new();
        for v in passages.keys() {
            seen.insert(*v, false);
        }
        Self {
            passages,
            target_position,
            seen,
            max_length: 0,
        }
    }

    fn dfs(&mut self, junction: Pos, length: usize) {
        if junction == self.target_position {
            if length > self.max_length {
                self.max_length = length;
            }
            return;
        }

        // In theory, we could have a junction that only has slopes leading
        // to it (follow_slopes == true for part 1), and thus has no passages
        // out of it.  So we wouldn't have an entry for it in `seen` and
        // `passages`.  But the input doesn't have this except for the
        // passage to `target_position` (which will return above), so ignore
        // this case.
        if self.seen[&junction] {
            return;
        }

        self.seen.insert(junction, true);

        let p = self.passages[&junction].to_vec();
        for (n, z) in p {
            self.dfs(n, length + z);
        }
        self.seen.insert(junction, false);
    }
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let part1_result = match part1(input, true) {
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
        let expected = "94";
        let actual = part1(file_data, true)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part1_bfs() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "94";
        let actual = part1(file_data, false)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "154";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
