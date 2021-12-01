use anyhow::Result;
use itertools::Itertools;

fn parse(input: &str) -> Result<Vec<u32>> {
    let result: Vec<u32> = input
        .lines()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    Ok(result)
}

fn part1(input: &str) -> Result<u32> {
    let parsed = parse(input)?;
    let result = parsed.iter().tuple_windows().fold(
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

fn part2(input: &str) -> Result<u32> {
    let parsed = parse(input)?;
    let sums: Vec<u32> = parsed
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
    println!("{}", part1(input)?);
    println!("{}", part2(input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = r#"199
200
208
210
200
207
240
269
260
263"#;

    #[test]
    fn test_part1() {
        let res = part1(INPUT).unwrap();
        assert_eq!(res, 7)
    }

    #[test]
    fn test_part2() {
        let res = part2(INPUT).unwrap();
        assert_eq!(res, 5)
    }
}
