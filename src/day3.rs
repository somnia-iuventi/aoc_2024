use std::{
    fs::File,
    io::{BufReader, Read},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn run(part: u8) {
    let file = File::open("src/inputs/3.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
        _ => return,
    };
}

#[derive(Debug, Clone)]
enum Command {
    Multiply(u64, u64),
    Do,
    Dont,
}

fn mult(input: &str) -> IResult<&str, Command> {
    let (rest, multiply) = delimited(
        tag("mul("),
        separated_pair(complete::u64, tag(","), complete::u64),
        tag(")"),
    )(input)?;
    Ok((rest, Command::Multiply(multiply.0, multiply.1)))
}

fn parse(input: &str) -> IResult<&str, Vec<Command>> {
    let dont = value(Command::Dont, tag("don't()"));
    let doo = value(Command::Do, tag("do()"));
    let (rest, answer) = many1(many_till(anychar, alt((doo, dont, mult))))(input)?;
    Ok((rest, answer.into_iter().map(|x| x.1).collect()))
}

pub fn part1(file: String) -> usize {
    let (_, parsed) = parse(file.as_str()).unwrap();
    parsed
        .iter()
        .filter_map(|x| match x {
            Command::Multiply(a, b) => Some(*a as usize * *b as usize),
            _ => None,
        })
        .sum()
}
pub fn part2(file: String) -> usize {
    let mut counting = true;
    let mut answer = 0;
    let (_, parsed) = parse(file.as_str()).unwrap();
    parsed.iter().for_each(|x| match x {
        Command::Multiply(a, b) => {
            if counting {
                answer += *a as usize * *b as usize;
            }
        }
        Command::Do => {
            counting = true;
        }
        Command::Dont => {
            counting = false;
        }
    });
    answer
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 161)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST2.to_owned()), 48)
    }
}
