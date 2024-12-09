use std::{
    fs::File,
    io::{BufReader, Read},
};

use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    combinator::opt,
    multi::{fold_many1, many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

pub fn run(part: u8) {
    let file = File::open("src/inputs/7.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
        _ => return,
    };
}

#[derive(Debug)]
struct Line {
    total: u64,
    inputs: Vec<u64>,
}

impl Line {
    fn new(total: u64, inputs: Vec<u64>) -> Self {
        Self { total, inputs }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    let inputs = terminated(
        fold_many1(
            preceded(space1, complete::u64),
            Vec::new,
            |mut acc, item| {
                acc.push(item);
                acc
            },
        ),
        opt(newline),
    );
    let single_line = separated_pair(complete::u64, tag(":"), inputs);
    let (rest, lines) = many1(single_line)(input)?;
    Ok((
        rest,
        lines
            .into_iter()
            .map(|(total, inputs)| Line::new(total, inputs))
            .collect(),
    ))
}

fn calculate_possible(line: &Line, operations: &Vec<char>) -> Option<u64> {
    let combos: Vec<Vec<&char>> = repeat_n(operations, line.inputs.len() - 1)
        .multi_cartesian_product()
        .collect();
    for combo in combos {
        let (_, total) = line
            .inputs
            .clone()
            .into_iter()
            .enumerate()
            .reduce(|a, b| match combo.get(a.0).unwrap() {
                '+' => (b.0, a.1 + b.1),
                '*' => (b.0, a.1 * b.1),
                '|' => (b.0, format!("{}{}", a.1, b.1).parse::<u64>().unwrap()),
                _ => (0, 0),
            })
            .unwrap();
        if line.total == 292 {
            dbg!(&combo);
            dbg!(total);
        }
        if total == line.total {
            return Some(line.total);
        }
    }
    None
}

pub fn part1(file: String) -> u64 {
    let operations = vec!['+', '*'];
    let (_, parsed) = parse_input(file.as_str()).unwrap();
    parsed
        .iter()
        .filter_map(|x| calculate_possible(x, &operations))
        .sum()
}
pub fn part2(file: String) -> u64 {
    let operations = vec!['+', '*', '|'];
    let (_, parsed) = parse_input(file.as_str()).unwrap();
    parsed
        .iter()
        .filter_map(|x| calculate_possible(x, &operations))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 3749)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST.to_owned()), 11387)
    }
}
