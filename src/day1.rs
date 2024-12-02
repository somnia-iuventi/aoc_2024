use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
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
    let reader = BufReader::new(file);
    let time = Instant::now();
    let thing = match part {
        1 => part1(reader),
        2 => part2(reader),
        _ => return,
    };
    dbg!(time.elapsed());
    dbg!(thing);
}

fn parse_line(input: &str) -> IResult<&str, (isize, isize)> {
    separated_pair(
        map(digit1, |x: &str| x.parse::<isize>().unwrap()),
        space0,
        map(digit1, |x: &str| x.parse::<isize>().unwrap()),
    )(input)
}

pub fn part1(file: BufReader<File>) -> isize {
    let mut vecs: (Vec<isize>, Vec<isize>) = file
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| parse_line(x.as_str()).unwrap().1)
        .unzip();
    vecs.1.sort();
    vecs.0.sort();
    zip(vecs.0, vecs.1).map(|(a, b)| (a - b).abs()).sum()
}
pub fn part2(file: BufReader<File>) -> isize {
    let vecs: (Vec<isize>, Vec<isize>) = file
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| parse_line(x.as_str()).unwrap().1)
        .unzip();
    vecs.0
        .iter()
        .map(|l| l * vecs.1.iter().filter(|x| *x == l).count() as isize)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_works() {
        let file = File::open("src/testinputs/1.txt").unwrap();
        let reader = BufReader::new(file);
        assert_eq!(part1(reader), 11)
    }
    #[test]
    fn part2_works() {
        let file = File::open("src/testinputs/1.txt").unwrap();
        let reader = BufReader::new(file);
        assert_eq!(part2(reader), 31)
    }
}
