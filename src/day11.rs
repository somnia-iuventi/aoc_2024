use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use itertools::Itertools;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn run(part: u8) {
    let file = File::open("src/inputs/11.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
        _ => return,
    };
}

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

fn parse_input_hm(input: &str) -> IResult<&str, HashMap<u64, usize>> {
    let parsed = separated_list1(space1, complete::u64)(input)?;
    Ok(("", HashMap::from_iter(parsed.1.iter().map(|x| (*x, 1)))))
}

fn apply_rules(num: u64, depth: u64, blinks: u64) -> Vec<u64> {
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
    if depth + 1 == blinks {
        return results;
    }
    return results
        .par_iter()
        // .iter()
        .flat_map(|x| apply_rules(*x, depth + 1, blinks))
        .collect::<Vec<u64>>();
}

fn blink_p2_v2(hm: &mut HashMap<u64, usize>) {
    let old = hm.iter().map(|x| (*x.0, *x.1)).collect_vec();
    hm.iter_mut().for_each(|x| *x.1 = 0);
    old.iter().for_each(|(num_on_stone, count)| {
        if *num_on_stone == 0 {
            hm.entry(1).and_modify(|x| *x += *count).or_insert(*count);
        } else if (num_on_stone.ilog10() + 1) % 2 == 0 {
            let length = num_on_stone.ilog10() + 1;
            let numstr = num_on_stone.to_string();
            let (first, sec) = numstr.split_at(length as usize / 2);
            hm.entry(first.parse().unwrap())
                .and_modify(|x| *x += *count)
                .or_insert(*count);
            hm.entry(sec.parse().unwrap())
                .and_modify(|x| *x += *count)
                .or_insert(*count);
        } else {
            hm.entry(*num_on_stone * 2024)
                .and_modify(|x| *x += *count)
                .or_insert(*count);
        }
    });
}

pub fn part1(file: String) -> usize {
    let (_, numbers) = parse_input(file.as_str()).unwrap();
    numbers
        .par_iter()
        .flat_map(|x| apply_rules(*x, 0, 25))
        .count()
}
pub fn part2(file: String) -> usize {
    let (_, mut numbers) = parse_input_hm(file.as_str()).unwrap();
    (0..75).into_iter().for_each(|_| {
        blink_p2_v2(&mut numbers);
    });
    numbers.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"125 17"#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 55312)
    }
}
