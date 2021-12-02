use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Default)]
struct SubmarineState {
    position: u32,
    depth: u32,
    aim: u32,
}

impl SubmarineState {
    fn sum(self) -> u32 {
        self.position * self.depth
    }
}

#[derive(Debug)]
enum SubmarineCommand {
    Forward(u32),
    Up(u32),
    Down(u32),
}

#[derive(Error, Debug)]
enum SubmarineError {
    #[error("Parsing command failed")]
    ParseCommand,
}

impl FromStr for SubmarineCommand {
    type Err = SubmarineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, amount) = s
            .split(' ')
            .collect_tuple()
            .ok_or(SubmarineError::ParseCommand)?;
        let amount: u32 = amount.parse().map_err(|_| SubmarineError::ParseCommand)?;
        let result = match command {
            "forward" => Self::Forward(amount),
            "up" => Self::Up(amount),
            "down" => Self::Down(amount),
            _ => {
                return Err(SubmarineError::ParseCommand);
            }
        };
        Ok(result)
    }
}

type SubmarinePlan = Vec<SubmarineCommand>;
type Input = [SubmarineCommand];

fn parse(input: &str) -> Result<SubmarinePlan, SubmarineError> {
    input
        .lines()
        .into_iter()
        .map(str::parse::<SubmarineCommand>)
        .collect()
}

fn part1(input: &Input) -> Result<u32> {
    let result = input
        .iter()
        .fold(SubmarineState::default(), |state, command| match command {
            SubmarineCommand::Forward(delta) => SubmarineState {
                position: state.position + delta,
                depth: state.depth,
                aim: 0,
            },
            SubmarineCommand::Up(delta) => SubmarineState {
                position: state.position,
                depth: state.depth - delta,
                aim: 0,
            },
            SubmarineCommand::Down(delta) => SubmarineState {
                position: state.position,
                depth: state.depth + delta,
                aim: 0,
            },
        });
    Ok(result.sum())
}

fn part2(input: &Input) -> Result<u32> {
    let result = input
        .iter()
        .fold(SubmarineState::default(), |state, command| match command {
            SubmarineCommand::Forward(delta) => SubmarineState {
                position: state.position + delta,
                depth: state.depth + (state.aim * delta),
                aim: state.aim,
            },
            SubmarineCommand::Up(delta) => SubmarineState {
                position: state.position,
                depth: state.depth,
                aim: state.aim - delta,
            },
            SubmarineCommand::Down(delta) => SubmarineState {
                position: state.position,
                depth: state.depth,
                aim: state.aim + delta,
            },
        });
    Ok(result.sum())
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let input = parse(input)?;
    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("input_test.txt");

    #[test]
    fn test_part1() {
        let res = part1(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 150)
    }

    #[test]
    fn test_part2() {
        let res = part2(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 900)
    }
}
