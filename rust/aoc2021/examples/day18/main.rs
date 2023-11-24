// cargo run --example day18 -- (part1|part2) (example_input|final_input)

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
#[structopt(name = "day18", about = "Advent of Code 2021 Day 18")]
pub struct Args {
    part: Part,
    input: Input,
}

#[derive(Debug, Clone, Copy)]
struct PairId(usize);

#[derive(Debug, Clone)]
struct Pair {
    left: PairEntry,
    right: PairEntry,
}

#[derive(Debug, Clone)]
enum PairEntry {
    Number(i32),
    Pair(PairId),
    ResolvedPair(Box<Pair>),
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl std::fmt::Display for PairEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PairEntry::Number(n) => write!(f, "{}", *n),
            PairEntry::Pair(_) => panic!(),
            PairEntry::ResolvedPair(p) => write!(f, "{}", **p),
        }
    }
}

#[derive(Default)]
struct Tree {
    nodes: Vec<Pair>,
}

impl Tree {
    fn push(&mut self, p: Pair) -> PairId {
        self.nodes.push(p);
        PairId(self.nodes.len() - 1)
    }

    fn get(&self, id: PairId) -> Pair {
        self.nodes.get(id.0).unwrap().clone()
    }

    fn set(&mut self, id: PairId, p: Pair) {
        self.nodes[id.0] = p;
    }

    #[cfg(test)]
    fn eq(&self, a: PairId, b: PairId) -> bool {
        if a.0 == b.0 {
            return true;
        }
        let (a, b) = (self.get(a), self.get(b));
        let left_match = match (a.left, b.left) {
            (PairEntry::Number(a), PairEntry::Number(b)) => a == b,
            (PairEntry::Pair(a), PairEntry::Pair(b)) => self.eq(a, b),
            _ => false,
        };
        if !left_match {
            return false;
        }
        let right_match = match (a.right, b.right) {
            (PairEntry::Number(a), PairEntry::Number(b)) => a == b,
            (PairEntry::Pair(a), PairEntry::Pair(b)) => self.eq(a, b),
            _ => false,
        };
        right_match
    }

    fn resolve(&self, id: PairId) -> Pair {
        let p = self.get(id);
        let left = match p.left {
            PairEntry::Pair(sp) => PairEntry::ResolvedPair(Box::new(self.resolve(sp))),
            PairEntry::ResolvedPair(_) => panic!(),
            v => v,
        };
        let right = match p.right {
            PairEntry::Pair(sp) => PairEntry::ResolvedPair(Box::new(self.resolve(sp))),
            PairEntry::ResolvedPair(_) => panic!(),
            v => v,
        };
        Pair { left, right }
    }

    fn unresolve(&mut self, p: Pair) -> PairId {
        let left = match p.left {
            PairEntry::Pair(_) => panic!(),
            PairEntry::ResolvedPair(sp) => PairEntry::Pair(self.unresolve(*sp)),
            v => v,
        };
        let right = match p.right {
            PairEntry::Pair(_) => panic!(),
            PairEntry::ResolvedPair(sp) => PairEntry::Pair(self.unresolve(*sp)),
            v => v,
        };
        self.push(Pair { left, right })
    }

    fn duplicate(&mut self, id: PairId) -> PairId {
        self.unresolve(self.resolve(id))
    }
}

fn add_without_reduce(t: &mut Tree, a: PairId, b: PairId) -> PairId {
    let pair = Pair {
        left: PairEntry::Pair(t.duplicate(a)),
        right: PairEntry::Pair(t.duplicate(b)),
    };
    t.push(pair)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
enum PathDir {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ReduceType {
    Explode,
    Split,
}

fn parse(t: &mut Tree, s: &str, pos: usize) -> PairId {
    let mut level = 0;
    let mut middle = 0;
    let mut end = 0;
    for (i, c) in s[pos..].chars().enumerate() {
        if c == ',' && level == 1 {
            middle = pos + i;
        }
        if c == ']' {
            level -= 1;
            if level == 0 {
                end = pos + i;
                break;
            }
        }
        if c == '[' {
            level += 1;
        }
    }
    let left = &s[pos + 1..middle];
    let right = &s[middle + 1..end];

    let left = if left.starts_with('[') {
        let pair = parse(t, s, pos + 1);
        PairEntry::Pair(pair)
    } else {
        PairEntry::Number(left.parse().unwrap())
    };

    let right = if right.starts_with('[') {
        let pair = parse(t, s, middle + 1);
        PairEntry::Pair(pair)
    } else {
        PairEntry::Number(right.parse().unwrap())
    };

    t.push(Pair { left, right })
}

fn add(t: &mut Tree, a: PairId, b: PairId) -> PairId {
    let res = add_without_reduce(t, a, b);
    reduce(t, res, false);
    res
}

fn sum_pairs(t: &mut Tree, l: Vec<PairId>) -> (PairId, Vec<PairId>) {
    let mut steps = Vec::new();
    let mut sum = None;
    for add in l {
        sum = match sum {
            Some(v) => Some(add_without_reduce(t, v, add)),
            None => Some(add),
        };
        reduce(t, sum.unwrap(), false);
        steps.push(t.duplicate(sum.unwrap()));
    }
    (sum.unwrap(), steps)
}

fn parse_lines(t: &mut Tree, contents: &str) -> Vec<PairId> {
    let mut pairids = Vec::new();
    for line in contents.lines() {
        pairids.push(parse(t, line.trim(), 0))
    }
    pairids
}

#[derive(Debug)]
struct Visit {
    current: PairId,
    parent: Option<(PairId, PathDir)>,
    depth: i32,
    left: Option<PairId>,
    right: Option<PairId>,
}

impl Visit {
    fn new(current: PairId, parent: Option<(PairId, PathDir)>, depth: i32) -> Self {
        Self {
            current,
            parent,
            depth,
            left: None,
            right: None,
        }
    }
}
fn flatten_inner(
    t: &Tree,
    r: PairId,
    parent: Option<(PairId, PathDir)>,
    depth: i32,
    visit: &mut Vec<Visit>,
) {
    let p = t.get(r);
    match p.left {
        PairEntry::Number(_) => visit.push(Visit::new(r, parent, depth)),
        PairEntry::Pair(sp) => flatten_inner(t, sp, Some((r, PathDir::Left)), depth + 1, visit),
        PairEntry::ResolvedPair(_) => panic!(),
    }
    match p.right {
        PairEntry::Number(_) => visit.push(Visit::new(r, parent, depth)),
        PairEntry::Pair(sp) => flatten_inner(t, sp, Some((r, PathDir::Right)), depth + 1, visit),
        PairEntry::ResolvedPair(_) => panic!(),
    }
}
fn flatten(t: &Tree, r: PairId) -> Vec<Visit> {
    let mut visit = Vec::new();
    flatten_inner(t, r, None, 1, &mut visit);
    for i in 0..visit.len() - 1 {
        visit[i].right = Some(visit[i + 1].current);
        visit[i + 1].left = Some(visit[i].current);
    }
    visit
}

fn reduce(t: &mut Tree, root: PairId, once: bool) -> bool {
    let mut again = true;
    while again {
        let visit = flatten(t, root);
        let explode_pass = (0..visit.len()).map(|x| (ReduceType::Explode, x));    
        let split_pass = (0..visit.len()).map(|x| (ReduceType::Split, x));    
        again = false;
        for (reduce_type, i) in explode_pass.chain(split_pass) {
            let v = &visit[i];
            let mut current = t.get(v.current);
            match reduce_type {
                ReduceType::Explode => {
                    if v.depth == 5 {
                        let left_val = if let PairEntry::Number(n) = current.left {
                            n
                        } else {
                            panic!();
                        };
                        let right_val = if let PairEntry::Number(n) = current.right {
                            n
                        } else {
                            panic!();
                        };

                        if let Some(id) = v.left {
                            let mut p = t.get(id);
                            if let PairEntry::Number(n) = p.right {
                                p.right = PairEntry::Number(n + left_val)
                            } else if let PairEntry::Number(n) = p.left {
                                p.left = PairEntry::Number(n + left_val);
                            }
                            t.set(id, p);
                        }
                        // There will be two visits for an exploding pair - both numbers...
                        let v = &visit[i + 1];
                        if let Some(id) = v.right {
                            let mut p = t.get(id);
                            if let PairEntry::Number(n) = p.left {
                                p.left = PairEntry::Number(n + right_val);
                            } else if let PairEntry::Number(n) = p.right {
                                p.right = PairEntry::Number(n + right_val);
                            }
                            t.set(id, p);
                        }
                        match v.parent {
                            Some((parentid, dir)) => {
                                let mut parent = t.get(parentid);
                                match dir {
                                    PathDir::Left => parent.left = PairEntry::Number(0),
                                    PathDir::Right => parent.right = PairEntry::Number(0),
                                };
                                t.set(parentid, parent);
                            }
                            None => panic!(),
                        }
                        again = true;
                        break;
                    }
                }
                ReduceType::Split => {
                    if let PairEntry::Number(n) = current.left {
                        if n > 9 {
                            let new_p = Pair {
                                left: PairEntry::Number(n / 2),
                                right: PairEntry::Number(n - (n / 2)),
                            };
                            current.left = PairEntry::Pair(t.push(new_p));
                            t.set(v.current, current);
                            again = true;
                            break;
                        }
                    }
                    if let PairEntry::Number(n) = current.right {
                        if n > 9 {
                            let new_p = Pair {
                                left: PairEntry::Number(n / 2),
                                right: PairEntry::Number(n - (n / 2)),
                            };
                            current.right = PairEntry::Pair(t.push(new_p));
                            t.set(v.current, current);
                            again = true;
                            break;
                        }
                    }
                }
            }
        }
        if once {
            return again;
        }
        again = again && !once;
    }
    false
}

fn magnitude(t: &Tree, pid: PairId) -> i32 {
    let pair = t.get(pid);
    let left = 3 * match pair.left {
        PairEntry::Number(n) => n,
        PairEntry::Pair(sp) => magnitude(t, sp),
        PairEntry::ResolvedPair(_) => panic!(),
    };
    let right = 2 * match pair.right {
        PairEntry::Number(n) => n,
        PairEntry::Pair(sp) => magnitude(t, sp),
        PairEntry::ResolvedPair(_) => panic!(),
    };
    left + right
}

fn largest_magnitude(t: &mut Tree, pids: Vec<PairId>) -> i32 {
    let mut result = 0;
    for i1 in 0..pids.len() {
        for i2 in i1 + 1..pids.len() {
            let s1 = add(t, pids[i1], pids[i2]);
            let s2 = add(t, pids[i2], pids[i1]);
            result = result.max(magnitude(t, s1)).max(magnitude(t, s2));
        }
    }
    result
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let mut t = Tree::default();
    let pids = parse_lines(&mut t, &contents);
    let (sum, _) = sum_pairs(&mut t, pids.clone());
    println!("{}", magnitude(&t, sum));
    println!("{}", largest_magnitude(&mut t, pids));

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day18/example_input",
        Input::FinalInput => "data/day18/input",
    };

    solve(file_path)
}

#[cfg(test)]
mod tests {
    use crate::{
        add_without_reduce, largest_magnitude, magnitude, parse, parse_lines,
        reduce, Tree, sum_pairs,
    };

    #[test]
    fn test_reduce_once() {
        let reduce_tests = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];
        let mut t = Tree::default();
        for (start, expected) in reduce_tests {
            let start_pid = parse(&mut t, start, 0);
            let expected_pid = parse(&mut t, expected, 0);
            reduce(&mut t, start_pid, true);
            assert_eq!(t.eq(expected_pid, start_pid), true);
        }
    }

    #[test]
    fn addition_with_manual_reduce() {
        let mut t = Tree::default();
        let a = parse(&mut t, "[[[[4,3],4],4],[7,[[8,4],9]]]", 0);
        let b = parse(&mut t, "[1,1]", 0);
        let expected = parse(&mut t, "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", 0);
        let c = add_without_reduce(&mut t, a, b);
        assert_eq!(t.eq(expected, c), true);

        let expected_reduces = [
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        ];
        let mut reduces = 0;
        while reduce(&mut t, c, true) {
            let expected = parse(&mut t, expected_reduces[reduces], 0);
            assert_eq!(t.eq(expected, c), true);
            reduces += 1;
        }
        let expected = parse(&mut t, expected_reduces.last().unwrap(), 0);
        assert_eq!(t.eq(expected, c), true);
    }

    #[test]
    fn sum() {
        let expected_sums = [
            (
                "[1,1]
                [2,2]
                [3,3]
                [4,4]",
                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            ),
            (
                "[1,1]
                [2,2]
                [3,3]
                [4,4]
                [5,5]",
                "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            ),
            (
                "[1,1]
                [2,2]
                [3,3]
                [4,4]
                [5,5]
                [6,6]",
                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            ),
        ];

        for (contents, expected) in expected_sums {
            let mut t = Tree::default();
            let pids = parse_lines(&mut t, contents);
            let (sum, _) = sum_pairs(&mut t, pids);
            let expected_pid = parse(&mut t, expected, 0);
            assert_eq!(t.eq(expected_pid, sum), true)
        }
    }

    #[test]
    fn manual_sum() {
        let contents = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
            [7,[5,[[3,8],[1,4]]]]
            [[2,[2,2]],[8,[8,1]]]
            [2,9]
            [1,[[[9,3],9],[[9,0],[0,7]]]]
            [[[5,[7,4]],7],1]
            [[[[4,2],2],6],[8,7]]";
        let expected_steps = [
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ];
        let mut t = Tree::default();
        let pids = parse_lines(&mut t, contents);
        let (_, steps) = sum_pairs(&mut t, pids);
        for (i, step) in steps.clone().into_iter().enumerate() {
            let expected_pid = parse(&mut t, expected_steps[i], 0);
            assert_eq!(t.eq(expected_pid, step), true)
        }
        assert_eq!(expected_steps.len(), steps.len())
    }

    #[test]
    fn example_part1() {
        let contents = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let expected = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]";
        let mut t = Tree::default();
        let pids = parse_lines(&mut t, contents);
        let (result, _) = sum_pairs(&mut t, pids);
        let expected_pid = parse(&mut t, expected, 0);
        assert!(t.eq(result, expected_pid));

        assert_eq!(magnitude(&t, result), 4140)
    }

    #[test]
    fn example_part2() {
        let contents = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let mut t = Tree::default();
        let pids = parse_lines(&mut t, contents);

        let result = largest_magnitude(&mut t, pids);
        assert_eq!(result, 3993)
    }
}
