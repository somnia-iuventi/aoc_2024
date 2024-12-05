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
    let reader = BufReader::new(file);
    match part {
        1 => dbg!(part1(reader)),
        2 => dbg!(part2(reader)),
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

pub fn part1(mut file: BufReader<File>) -> usize {
    let mut whole_file = String::new();
    file.read_to_string(&mut whole_file).unwrap();
    let (_, parsed) = parse(whole_file.as_str()).unwrap();
    parsed
        .iter()
        .filter_map(|x| match x {
            Command::Multiply(a, b) => Some(*a as usize * *b as usize),
            _ => None,
        })
        .sum()
}
pub fn part2(mut file: BufReader<File>) -> usize {
    let mut whole_file = String::new();
    let mut counting = true;
    let mut answer = 0;
    file.read_to_string(&mut whole_file).unwrap();
    let (_, parsed) = parse(whole_file.as_str()).unwrap();
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
    #[test]
    fn part1_works() {
        let file = File::open("src/testinputs/3.txt").unwrap();
        let reader = BufReader::new(file);
        assert_eq!(part1(reader), 161)
    }
    #[test]
    fn part2_works() {
        let file = File::open("src/testinputs/3-2.txt").unwrap();
        let reader = BufReader::new(file);
        assert_eq!(part2(reader), 48)
    }
}
