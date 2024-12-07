use std::{
    fs::File,
    io::{BufReader, Read},
};

use nom::{
    character::complete::{digit1, space0},
    combinator::map,
    sequence::separated_pair,
    IResult,
};
use std::iter::zip;

pub fn run(part: u8) {
    let file = File::open("src/inputs/1.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    let thing = match part {
        1 => part1(whole_file),
        2 => part2(whole_file),
        _ => return,
    };
    dbg!(thing);
}

fn parse_line(input: &str) -> IResult<&str, (isize, isize)> {
    separated_pair(
        map(digit1, |x: &str| x.parse::<isize>().unwrap()),
        space0,
        map(digit1, |x: &str| x.parse::<isize>().unwrap()),
    )(input)
}

pub fn part1(file: String) -> isize {
    let mut vecs: (Vec<isize>, Vec<isize>) = file.lines().map(|x| parse_line(x).unwrap().1).unzip();
    vecs.1.sort();
    vecs.0.sort();
    zip(vecs.0, vecs.1).map(|(a, b)| (a - b).abs()).sum()
}
pub fn part2(file: String) -> isize {
    let vecs: (Vec<isize>, Vec<isize>) = file.lines().map(|x| parse_line(x).unwrap().1).unzip();
    vecs.0
        .iter()
        .map(|l| l * vecs.1.iter().filter(|x| *x == l).count() as isize)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 11)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST.to_owned()), 31)
    }
}
