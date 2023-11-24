// cargo run --example day24 -- (part1|part2) (example_input|final_input)

use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Part {
    Part1,
    Part2,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Input {
    ExampleInput,
    FinalInput,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day24", about = "Advent of Code 2021 Day 24")]
pub struct Args {
    part: Part,
    input: Input,
}

struct Program {
    instructions: Vec<Instruction>,
}

#[derive(Debug, strum::EnumString, Clone, Copy)]
#[strum(serialize_all = "snake_case")]
enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Variable(usize),
    Number(i64),
}

impl Default for Operand {
    fn default() -> Self {
        Operand::Number(i64::MAX)
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct State {
    variables: [i64; 4],
    pc: usize,
}

fn parse_arg(arg: &str) -> Operand {
    let variable_map = HashMap::from([("w", 0), ("x", 1), ("y", 2), ("z", 3)]);
    if let Some(v) = variable_map.get(arg) {
        return Operand::Variable(*v);
    }
    let n = arg.parse::<i64>().unwrap();
    Operand::Number(n)
}

fn compile(contents: &str) -> Program {
    let lines = contents.lines();
    let mut instructions = Vec::new();

    for mut line in lines {
        line = line.trim();
        let (instr, rest) = line.split_once(' ').unwrap();
        let args: Vec<_> = rest.split_whitespace().map(|a| parse_arg(a)).collect();
        let mut instruction = Instruction::from_str(instr).unwrap();
        match &mut instruction {
            Instruction::Inp(o) => *o = args[0],
            Instruction::Add(o1, o2)
            | Instruction::Mul(o1, o2)
            | Instruction::Div(o1, o2)
            | Instruction::Mod(o1, o2)
            | Instruction::Eql(o1, o2) => {
                *o1 = args[0];
                *o2 = args[1];
            }
        }
        instructions.push(instruction);
    }
    Program { instructions }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ExecuteResult {
    Ok,
    NeedNextInput,
    ConsumedInput,
    EndOfInstructions,
}

fn execute_instruction(
    instruction: Instruction,
    state: &mut State,
    input: Option<i64>,
) -> ExecuteResult {
    let mut execute_result = ExecuteResult::Ok;
    match instruction {
        Instruction::Inp(o) => match o {
            Operand::Variable(v) => {
                if let Some(input) = input {
                    state.variables[v] = input;
                    execute_result = ExecuteResult::ConsumedInput;
                } else {
                    return ExecuteResult::NeedNextInput;
                }
            }
            Operand::Number(_) => panic!(),
        },
        Instruction::Add(o1, o2) => {
            let rhs = match o2 {
                Operand::Variable(v) => state.variables[v],
                Operand::Number(n) => n,
            };
            match o1 {
                Operand::Variable(v) => state.variables[v] += rhs,
                Operand::Number(_) => panic!(),
            }
        }
        Instruction::Mul(o1, o2) => {
            let rhs = match o2 {
                Operand::Variable(v) => state.variables[v],
                Operand::Number(n) => n,
            };
            match o1 {
                Operand::Variable(v) => state.variables[v] *= rhs,
                Operand::Number(_) => panic!(),
            }
        }
        Instruction::Div(o1, o2) => {
            let rhs = match o2 {
                Operand::Variable(v) => state.variables[v],
                Operand::Number(n) => n,
            };
            match o1 {
                Operand::Variable(v) => state.variables[v] /= rhs,
                Operand::Number(_) => panic!(),
            }
        }
        Instruction::Mod(o1, o2) => {
            let rhs = match o2 {
                Operand::Variable(v) => state.variables[v],
                Operand::Number(n) => n,
            };
            match o1 {
                Operand::Variable(v) => state.variables[v] %= rhs,
                Operand::Number(_) => panic!(),
            }
        }
        Instruction::Eql(o1, o2) => {
            let rhs = match o2 {
                Operand::Variable(v) => state.variables[v],
                Operand::Number(n) => n,
            };
            match o1 {
                Operand::Variable(v) => state.variables[v] = (state.variables[v] == rhs) as i64,
                Operand::Number(_) => panic!(),
            }
        }
    }
    state.pc += 1;
    execute_result
}

fn execute_until_input(
    program: &Program,
    state: &mut State,
    mut input: Option<i64>,
) -> ExecuteResult {
    // Use the input, if present, and then execute until the next input statement.
    let mut execute_result = ExecuteResult::EndOfInstructions;
    while let Some(&instruction) = program.instructions.get(state.pc) {
        let r = execute_instruction(instruction, state, None);
        if let ExecuteResult::NeedNextInput = r {
            if let Some(next_input) = input {
                let r = execute_instruction(instruction, state, Some(next_input));
                input = None;
                assert_eq!(r, ExecuteResult::ConsumedInput);
            } else {
                execute_result = r;
                break;
            }
        }
    }
    execute_result
}

#[cfg(test)]
fn execute(program: Program, inputs: &[i64]) -> State {
    let mut state = State::default();
    let mut input_position = 0;
    let mut next_input = || {
        let res = inputs[input_position];
        input_position += 1;
        res
    };

    loop {
        let r = execute_until_input(&program, &mut state, Some(next_input()));
        if r == ExecuteResult::EndOfInstructions {
            break;
        }
    }
    state
}

fn search<const T: bool>(
    program: &Program,
    state: State,
    memo: &mut HashMap<(usize, i64), Option<i64>>,
) -> Option<i64> {
    let res = _search::<T>(program, state, memo);
    res.map(|r| {
        r.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap()
    })
}

fn _search<const T: bool>(
    program: &Program,
    state: State,
    memo: &mut HashMap<(usize, i64), Option<i64>>,
) -> Option<i64> {
    let key = (state.pc, state.variables[3]);
    if let Some(&result) = memo.get(&key) {
        return result;
    }
    let inputs = if T {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    };
    for input in inputs {
        let mut state = state;
        let execute_result = execute_until_input(program, &mut state, Some(input));
        let key = (state.pc, state.variables[3]);
        if execute_result == ExecuteResult::NeedNextInput {
            if let Some(found) = _search::<T>(program, state, memo) {
                let result = found * 10 + input;
                memo.insert(key, Some(result));
                return Some(result);
            }
        }

        if state.variables[3] == 0 {
            memo.insert(key, Some(input));
            return Some(input);
        }
    }

    memo.insert(key, None);
    None
}

fn solve(file_path: &str) -> Result<()> {
    let contents = read_to_string(file_path)?;

    let program = compile(&contents);

    {
        let state = State::default();
        let mut memo = HashMap::new();
        let biggest = search::<true>(&program, state, &mut memo);
        println!("Part 1: {:?}", biggest);
    }

    {
        let state = State::default();
        let mut memo = HashMap::new();
        let smallest = search::<false>(&program, state, &mut memo);
        println!("Part 2: {:?}", smallest);
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::from_args_safe()?;

    let file_path = match args.input {
        Input::ExampleInput => "data/day24/example_input",
        Input::FinalInput => "data/day24/input",
    };

    solve(file_path)
}

#[cfg(test)]
fn compute(contents: &str, inputs: &[i64]) -> State {
    let program = compile(contents);
    execute(program, inputs)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::compute;

    #[test]
    fn test_example_1() {
        let program = "inp x
        mul x -1";
        let inputs = [5];
        let res = compute(program, &inputs);
        println!("{:?}", res);
        assert_eq!(res.variables[1], -5);
    }

    #[test]
    fn test_example_2() {
        let program = "inp z
        inp x
        mul z 3
        eql z x";
        let inputs = [5, 15];
        let res = compute(program, &inputs);
        println!("{:?}", res);
        assert_eq!(res.variables[1], 15);
        assert_eq!(res.variables[3], 1);
    }

    #[test]
    fn test_example_3() {
        let program = "inp w
            add z w
            mod z 2
            div w 2
            add y w
            mod y 2
            div w 2
            add x w
            mod x 2
            div w 2
            mod w 2";
        let inputs = [i64::from_str_radix("1101", 2).unwrap()];
        let res = compute(program, &inputs);
        println!("{:?}", res);
        assert_eq!(res.variables[0], 1);
        assert_eq!(res.variables[1], 1);
        assert_eq!(res.variables[2], 0);
        assert_eq!(res.variables[3], 1);
    }

    #[test]
    fn test_part_a() {
        let program = read_to_string("data/day24/input").unwrap();
        let inputs: Vec<_> = "71143112161181"
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        println!("{:?}", inputs);
        println!("{:?}", inputs.len());
        let res = compute(&program, inputs.as_slice());
        println!("{:?}", res);
    }

    #[test]
    fn test_part_a_first() {
        let program = "inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 14
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 12
        mul y x
        add z y";
        let inputs: Vec<_> = "13579246899999"
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        let res = compute(&program, inputs.as_slice());
        println!("{:?}", res);
    }
}
