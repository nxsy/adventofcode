use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("unimplemented")]
struct Unimplemented;

fn process(input: &str, part2: bool) -> Result<String> {
    let mut total = 0;
    for line in input.lines() {
        let mut seq = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        if part2 {
            seq.reverse();
        }

        let mut stack = Vec::new();
        let mut p = seq.clone();
        while !p.iter().all(|n| *n == 0) {
            let mut new_p = Vec::new();
            for i in 1..p.len() {
                let a = p[i - 1];
                let b = p[i];
                new_p.push(b - a);
            }
            stack.push(p);
            p = new_p;
        }
        stack.push(vec![0; stack.last().unwrap().len()]);

        for n in (0..(stack.len() - 1)).rev() {
            let this_line_length = stack[n].len() - 1;
            let end_of_this_line = stack[n][this_line_length];
            let end_of_last_line = stack[n + 1][this_line_length];
            stack[n].push(end_of_this_line + end_of_last_line);
        }
        total += stack[0].last().unwrap();
    }
    Ok(total.to_string())
}

fn part1(input: &str) -> Result<String> {
    process(input, false)
}

fn part2(input: &'static str) -> Result<String> {
    process(input, true)
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
        let expected_each = ["18", "28", "68"];
        for (i, line) in file_data.lines().enumerate() {
            let expected = expected_each[i];
            let actual = part1(line)?;
            assert_eq!(actual, expected);
        }
        let expected = "114";
        let actual = part1(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let file_data = include_str!("example_input.txt");
        let expected_each = ["-3", "0", "5"];
        for (i, line) in file_data.lines().enumerate() {
            let expected = expected_each[i];
            let actual = part2(line)?;
            assert_eq!(actual, expected);
        }
        let expected = "2";
        let actual = part2(file_data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
