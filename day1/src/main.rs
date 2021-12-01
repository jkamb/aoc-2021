use anyhow::Result;
use itertools::Itertools;

type Input = [u32];

fn parse(input: &str) -> Result<Vec<u32>> {
    let result: Vec<u32> = input
        .lines()
        .into_iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    Ok(result)
}

fn part1(input: &Input) -> Result<u32> {
    let result = input.iter().tuple_windows().fold(
        0,
        |acc, (prev, next)| {
            if next > prev {
                acc + 1
            } else {
                acc
            }
        },
    );
    Ok(result)
}

fn part2(input: &Input) -> Result<u32> {
    let sums: Vec<u32> = input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect();
    let result = sums.iter().tuple_windows().fold(
        0,
        |acc, (prev, next)| {
            if next > prev {
                acc + 1
            } else {
                acc
            }
        },
    );
    Ok(result)
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
        assert_eq!(res, 7)
    }

    #[test]
    fn test_part2() {
        let res = part2(&parse(INPUT).unwrap()).unwrap();
        assert_eq!(res, 5)
    }
}
