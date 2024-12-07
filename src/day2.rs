use std::{
    fs::File,
    io::{BufReader, Read},
};

use itertools::Itertools;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

pub fn run(part: u8) {
    let file = File::open("src/inputs/2.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
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

pub fn part1(file: String) -> usize {
    file.lines()
        .map(|x| parse_line(x).unwrap().1)
        .map(|x| safe_report(&x))
        .filter(|x| *x == true)
        .count()
}
pub fn part2(file: String) -> usize {
    file.lines()
        .map(|x| parse_line(x).unwrap().1)
        .map(|x| safe_report_p2(&x))
        .filter(|x| *x == true)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 2)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST.to_owned()), 4)
    }
}
