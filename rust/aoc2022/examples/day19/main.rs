//! Day 19

use std::str::FromStr;

use adventofcode2022::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u64 as nom_u64},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    part: Part,
    #[arg(long)]
    input: Input,
}

#[derive(Debug, strum::EnumString, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
#[allow(clippy::enum_variant_names)]
#[strum(serialize_all = "snake_case")]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Recipe {
    resource: Resource,
    // cost: Vec<(Resource, u64)>,
    cost: [u64; 4],
}

impl Recipe {
    fn can_buy(&self, ores: &[u64; 4]) -> bool {
        for (i, c) in self.cost.iter().enumerate() {
            if ores[i] < *c {
                return false;
            }
        }
        true
    }

    fn buy(&self, state: &mut State) {
        for (i, c) in self.cost.iter().enumerate() {
            state.ore[i] -= *c;
        }
        state.robots[self.resource as usize] += 1;
    }
}

#[derive(Debug)]
struct Blueprint {
    recipes: [Recipe; 4],
    max_costs: [u64; 4],
}

fn parse_phrase(input: &str) -> IResult<&str, Recipe> {
    let (input, (resource_name, costs)) = tuple((
        preceded(tag("Each "), alpha1),
        delimited(
            tag(" robot costs "),
            separated_list1(tag(" and "), separated_pair(nom_u64, tag(" "), alpha1)),
            tag("."),
        ),
    ))(input)?;
    let mut cost = [0; 4];
    for (n, r) in costs {
        let r = Resource::from_str(r).unwrap();
        cost[r as usize] = n;
    }
    Ok((
        input,
        Recipe {
            resource: Resource::from_str(resource_name).unwrap(),
            cost,
        },
    ))
}

fn parse_line(input: &str) -> IResult<&str, Blueprint> {
    let (input, recipes) = preceded(
        tuple((tag("Blueprint "), nom_u64, tag(": "))),
        separated_list1(tag(" "), parse_phrase),
    )(input)?;
    let mut max_costs = [0; 4];
    let recipes: [Recipe; 4] = recipes.try_into().unwrap();
    for r in &recipes {
        max_costs[0] = max_costs[0].max(r.cost[0]);
        max_costs[1] = max_costs[1].max(r.cost[1]);
        max_costs[2] = max_costs[2].max(r.cost[2]);
        max_costs[3] = max_costs[3].max(r.cost[3]);
    }
    Ok((
        input,
        Blueprint {
            recipes, // : recipes.into_iter().map(|x| (x.resource, x)).collect(),
            max_costs,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Blueprint>> {
    all_consuming(separated_list1(line_ending, parse_line))(input)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct State {
    robots: [u64; 4],
    ore: [u64; 4],
    // minutes_left: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            ore: [0; 4],
        }
    }
}

impl State {
    fn dig(&mut self) {
        for (i, &r) in self.robots.iter().enumerate() {
            self.ore[i] += r;
        }
    }
}

fn part1(file_data: &str) -> Result<()> {
    let (_, blueprints) = parse_input(file_data).unwrap();
    let minutes = 24;

    let geodes = determine_quality_levels(&blueprints, minutes);
    let quality_level = geodes
        .iter()
        .enumerate()
        .map(|(i, g)| g * (i as u64 + 1))
        .sum::<u64>();
    dbg!(quality_level);
    Ok(())
}

fn determine_quality_levels(blueprints: &[Blueprint], minutes: i32) -> Vec<u64> {
    let mut geodes = Vec::new();
    for Blueprint { recipes, max_costs } in blueprints {
        let initial_state = State::default();

        let mut seen = HashSet::new();
        let mut queue = VecDeque::from([(initial_state, minutes)]);
        let mut most_geode_robots = 0;
        let mut max_geodes = 0;

        while let Some((state, minutes_left)) = queue.pop_front() {
            if minutes_left == 0
                || seen.contains(&state)
                || state.robots[Resource::Geode as usize] < most_geode_robots
            {
                max_geodes = max_geodes.max(state.ore[Resource::Geode as usize]);
                continue;
            }

            most_geode_robots = most_geode_robots.max(state.robots[Resource::Geode as usize]);
            seen.insert(state.clone());

            if recipes[Resource::Geode as usize].can_buy(&state.ore) {
                let mut new_state = state.clone();
                new_state.dig();
                recipes[Resource::Geode as usize].buy(&mut new_state);
                queue.push_back((new_state, minutes_left - 1));
                // Always best to buy geode robot?
                continue;
            }

            for r in Resource::iter() {
                if r == Resource::Geode {
                    continue;
                }
                if state.robots[r as usize] > max_costs[r as usize] {
                    continue;
                }
                if recipes[r as usize].can_buy(&state.ore) {
                    let mut new_state = state.clone();
                    new_state.dig();
                    recipes[r as usize].buy(&mut new_state);
                    queue.push_back((new_state, minutes_left - 1));
                }
            }

            {
                let mut new_state = state.clone();
                new_state.dig();
                queue.push_back((new_state, minutes_left - 1));
            }
        }
        geodes.push(max_geodes);
    }
    geodes
}

fn part2(file_data: &str) -> Result<()> {
    let (_, blueprints) = parse_input(file_data).unwrap();
    let minutes = 32;

    let geodes = determine_quality_levels(&blueprints[0..3], minutes);
    let product = geodes.iter().product::<u64>();
    dbg!(product);
    dbg!(geodes);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Args: {:#?}", args);

    let file_data = match args.input {
        Input::ExampleInput => include_str!("example_input"),
        Input::FinalInput => include_str!("input"),
    };
    match args.part {
        Part::Part1 => part1(file_data),
        Part::Part2 => part2(file_data),
    }
}
