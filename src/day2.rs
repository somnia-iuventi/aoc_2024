use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

pub fn run(part: u8) {
    let file = File::open("src/inputs/2.txt").unwrap();
    let reader = BufReader::new(file);
    match part {
        1 => dbg!(part1(reader)),
        2 => dbg!(part2(reader)),
        _ => return,
    };
}
fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, complete::i64)(input)
}

fn safe_report(single_report: &Vec<i64>) -> bool {
    let diffs = single_report.windows(2).map(|x| x[0] - x[1]);
    return diffs.clone().map(|x| x.signum()).dedup().count() == 1
        && diffs.filter(|x| x.abs() > 3 || x.abs() < 1).count() == 0;
}

fn safe_report_p2(single_report: &Vec<i64>) -> bool {
    for ind in 0..single_report.len() {
        let diffs = single_report
            .iter()
            .enumerate()
            .filter(|x| x.0 != ind)
            .map(|x| x.1)
            .tuple_windows::<(&i64, &i64)>()
            .map(|x| x.0 - x.1);

        if diffs.clone().map(|x| x.signum()).dedup().count() == 1
            && diffs.filter(|x| x.abs() > 3 || x.abs() < 1).count() == 0
        {
            return true;
        }
    }
    false
}

pub fn part1(file: BufReader<File>) -> usize {
    file.lines()
        .filter_map(|x| x.ok())
        .map(|x| parse_line(x.as_str()).unwrap().1)
        .map(|x| safe_report(&x))
        .filter(|x| *x == true)
        .count()
}
pub fn part2(file: BufReader<File>) -> usize {
    file.lines()
        .filter_map(|x| x.ok())
        .map(|x| parse_line(x.as_str()).unwrap().1)
        .map(|x| safe_report_p2(&x))
        .filter(|x| *x == true)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_works() {
        let file = File::open("src/testinputs/2.txt").unwrap();
        let reader = BufReader::new(file);
        assert_eq!(part1(reader), 2)
    }
    #[test]
    fn part2_works() {
        let file = File::open("src/testinputs/2.txt").unwrap();
        let reader = BufReader::new(file);
        assert_eq!(part2(reader), 4)
    }
}
