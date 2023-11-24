//! Day 16
//!
//! Ugh.

use adventofcode2022::prelude::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, u64 as nom_u64},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

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

struct Valve {
    flow_rate: u64,
    tunnels: HashSet<String>,
}

impl Valve {
    fn new(flow_rate: u64, tunnels: Vec<&str>) -> Self {
        Self {
            flow_rate,
            tunnels: tunnels.into_iter().map(|x| x.to_owned()).collect(),
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, HashMap<&str, Valve>> {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    let (input, valve_data) = separated_list1(
        line_ending,
        tuple((
            preceded(tag("Valve "), alpha1),
            preceded(tag(" has flow rate="), nom_u64),
            preceded(
                alt((
                    tag("; tunnels lead to valves "),
                    tag("; tunnel leads to valve "),
                )),
                separated_list1(tag(", "), alpha1),
            ),
        )),
    )(input)?;
    Ok((
        input,
        valve_data
            .into_iter()
            .map(|(name, flow_rate, tunnels)| (name, Valve::new(flow_rate, tunnels)))
            .collect(),
    ))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct State {
    pos: u64,
    valves_open: Bitset,
    openable_valves: Bitset,
    pressure_relieved: u64,
    minutes_left: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Bitset(u64);

impl Bitset {
    fn is_set(&self, pos: u64) -> bool {
        assert!(pos < 64);
        let bit_pos = 1 << pos;
        (self.0 & bit_pos) == bit_pos
    }

    fn set(&mut self, pos: u64) {
        assert!(pos < 64);
        let bit_pos = 1 << pos;
        self.0 |= bit_pos
    }

    fn unset(&mut self, pos: u64) {
        assert!(pos < 64);
        let bit_pos = 1 << pos;
        self.0 &= !bit_pos;
    }

    fn all_unset(&self) -> bool {
        self.0 == 0
    }

    fn set_positions(&self) -> Vec<u64> {
        let mut t = self.0;
        let mut res: Vec<u64> = Vec::new();
        while t != 0 {
            let pos = t.trailing_zeros();
            res.push(pos as u64);
            t -= 1 << pos;
        }
        res
    }
}

fn part1(file_data: &str) -> Result<()> {
    let (_, valves) = parse_input(file_data).unwrap();
    let (valves_by_number, openable_valves) = extract_valve_data(valves);

    let mut memo = HashMap::new();

    let state = State {
        pos: 0,
        valves_open: Bitset(0),
        openable_valves: Bitset(openable_valves),
        pressure_relieved: 0,
        minutes_left: 30,
    };
    let max_relieved = maximize_relieved_pressure(state, &valves_by_number, &mut memo);

    dbg!(max_relieved);
    Ok(())
}

fn extract_valve_data(valves: HashMap<&str, Valve>) -> (Vec<(u64, Bitset)>, u64) {
    let mut valves_by_number: Vec<(u64, Bitset)> = Vec::new();
    let mut valve_names = valves.iter().map(|x| x.0.to_string()).collect::<Vec<_>>();
    valve_names.sort_unstable();
    for name in valve_names.iter() {
        let v = &valves[name.as_str()];
        let tunnels_by_number = v
            .tunnels
            .iter()
            .map(|x| 1 << valve_names.binary_search(x).unwrap())
            .sum::<u64>();
        valves_by_number.push((v.flow_rate, Bitset(tunnels_by_number)));
    }
    let openable_valves = valves_by_number
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if v.0 > 0 { Some(1 << i) } else { None })
        .sum::<u64>();
    (valves_by_number, openable_valves)
}

fn maximize_relieved_pressure(
    state: State,
    valves_by_number: &[(u64, Bitset)],
    memo: &mut HashMap<State, (u64, State)>,
) -> (u64, State) {
    if let Some(relieved_pressure) = memo.get(&state) {
        return (relieved_pressure.0, relieved_pressure.1.clone());
    }
    let best_state = (|| {
        if state.openable_valves.all_unset() {
            return (state.pressure_relieved, state.clone());
        }
        if state.minutes_left == 1 {
            // Nothing we do now will make a difference?
            return (state.pressure_relieved, state.clone());
        }

        let mut best_state = None;

        let valve = &valves_by_number[state.pos as usize];

        if !state.valves_open.is_set(state.pos) && state.openable_valves.is_set(state.pos) {
            let mut new_state = state.clone();
            new_state.minutes_left -= 1;
            new_state.valves_open.set(state.pos);
            new_state.openable_valves.unset(state.pos);
            new_state.pressure_relieved += valve.0 * new_state.minutes_left;
            best_state = Some(maximize_relieved_pressure(
                new_state,
                valves_by_number,
                memo,
            ));
        }

        for destination in valve.1.set_positions() {
            let mut new_state = state.clone();
            new_state.minutes_left -= 1;
            new_state.pos = destination;
            let local_state = maximize_relieved_pressure(new_state, valves_by_number, memo);
            best_state = Some(match best_state {
                None => local_state,
                Some(s) => {
                    if s.0 > local_state.0 {
                        s
                    } else {
                        local_state
                    }
                }
            })
        }
        best_state.unwrap()
    })();
    memo.insert(state, best_state.clone());
    best_state
}

fn part2(file_data: &str) -> Result<()> {
    let (_, valves) = parse_input(file_data).unwrap();
    let (valves_by_number, openable_valves) = extract_valve_data(valves);

    let mut memo = HashMap::new();

    let state = State {
        pos: 0,
        valves_open: Bitset(0),
        openable_valves: Bitset(openable_valves),
        pressure_relieved: 0,
        minutes_left: 26,
    };
    let max_relieved = maximize_relieved_pressure(state, &valves_by_number, &mut memo);

    let mut state = max_relieved.1;
    state.minutes_left = 26;
    state.pos = 0;
    let max_relieved = maximize_relieved_pressure(state, &valves_by_number, &mut memo);

    dbg!(max_relieved);
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

#[cfg(test)]
mod tests {
    #[test]
    fn example_part1() {}
}
