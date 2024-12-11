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
use rayon::prelude::*;

pub fn run(part: u8) {
    let file = File::open("src/inputs/11.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        // 2 => dbg!(part2(whole_file)),
        _ => return,
    };
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

fn apply_rules(num: u64, depth: u64, max_depth: u64) -> Vec<u64> {
    let num_string = num.to_string();
    let results = if num == 0 {
        vec![1]
    } else if num_string.len() % 2 == 0 {
        let split_index = num_string.len() / 2;
        let ret = num_string.split_at(split_index);
        vec![ret.0.parse::<u64>().unwrap(), ret.1.parse::<u64>().unwrap()]
    } else {
        vec![num * 2024]
    };
    if depth == 75 {
        return results;
    }
    return results
        // .par_iter()
        .iter()
        .flat_map(|x| apply_rules(*x, depth + 1, max_depth))
        .collect::<Vec<u64>>();
}

pub fn part1(file: String) -> usize {
    let (_, mut numbers) = parse_input(file.as_str()).unwrap();
    numbers = numbers
        .par_iter()
        .flat_map(|x| apply_rules(*x, 0, 25))
        .collect::<Vec<u64>>();
    numbers.len()
}
// pub fn part2(file: String) -> usize {
//     let (_, mut numbers) = parse_input(file.as_str()).unwrap();
//     (0..75).into_iter().for_each(|_| {
//         numbers = numbers
//             .par_iter()
//             .flat_map(|x| apply_rules_p1(*x))
//             .collect::<Vec<u64>>();
//     });
//     numbers.len()
// }

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"125 17"#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 55312)
    }
    // #[test]
    // fn part2_works() {
    //     assert_eq!(part2(TEST.to_owned()), 0)
    // }
}
