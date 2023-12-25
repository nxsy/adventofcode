use itertools::Itertools;
use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::graph::UnGraph};
use std::{
    collections::{BTreeMap, BTreeSet},
    iter::once,
};

use anyhow::{Context, Result};

fn part1(input: &'static str) -> Result<String> {
    let components: BTreeMap<&str, Vec<&str>> = input
        .lines()
        .map(|l| {
            let (n, components) = l.split_once(": ").context("missing colon")?;
            let components = components.split(' ').collect();
            Ok((n, components))
        })
        .collect::<Result<_>>()?;

    let name_to_id: BTreeMap<&str, u32> = components
        .iter()
        .flat_map(|c| once(*c.0).chain(c.1.iter().copied()))
        .unique()
        .enumerate()
        .map(|(i, n)| (n, i as u32))
        .collect();

    let edges: BTreeSet<(u32, u32)> = components
        .iter()
        .flat_map(|(k, v)| v.iter().map(move |v| (k, v)))
        .map(|(k, v)| {
            let (k, v) = (name_to_id[k], name_to_id[v]);
            (k.min(v), v.max(k))
        })
        .collect();

    let graph: UnGraph<(), ()> = UnGraph::from_edges(edges);

    let min_cut_res: Result<_> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let min_cut = min_cut_res?.context("no min cut found")?;

    let result = (name_to_id.len() - min_cut.1.len()) * min_cut.1.len();

    Ok(result.to_string())
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected = "54";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
