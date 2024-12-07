use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::opt,
    multi::{many1, many_till, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

pub fn run(part: u8) {
    let file = File::open("src/inputs/5.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
        _ => return,
    };
}

struct Rule {
    before: u64,
    after: u64,
}

impl Rule {
    fn new(before: u64, after: u64) -> Self {
        Self { before, after }
    }
}

struct Rules(Vec<Rule>);

impl Rules {
    fn new(input: Vec<(u64, u64)>) -> Self {
        Self(input.into_iter().map(|x| Rule::new(x.0, x.1)).collect())
    }

    fn add_error_vals(&self, input: u64) -> Vec<u64> {
        self.0
            .iter()
            .filter(|x| x.after == input)
            .map(|x| x.before)
            .collect::<Vec<u64>>()
    }
    fn add_error_vals_by_key(&self, input: u64, hm: &mut HashMap<u64, Vec<u64>>) {
        self.0
            .iter()
            .filter(|x| x.after == input)
            .map(|x| x.before)
            .for_each(|x| {
                let thing = hm.entry(x).or_default();
                thing.push(input);
            });
    }
}

struct Manual(Vec<u64>);
impl Manual {
    fn new(input: Vec<u64>) -> Self {
        Self(input)
    }

    fn is_good(&self, rules: &Rules) -> Option<u64> {
        let mut bad_numbers = Vec::new();
        for num in self.0.iter() {
            if bad_numbers.contains(num) {
                return None;
            }
            bad_numbers.append(&mut rules.add_error_vals(*num));
        }
        Some(*self.0.get(self.0.len() / 2).unwrap())
    }

    fn self_fixing(&self, rules: &Rules) -> Option<u64> {
        let mut bad_numbers_map = HashMap::new();
        let mut ordered_manual = Vec::new();
        let mut was_bad = false;
        for num in self.0.iter() {
            if !bad_numbers_map.keys().contains(num) {
                ordered_manual.push(*num);
                rules.add_error_vals_by_key(*num, &mut bad_numbers_map);
                continue;
            }
            was_bad = true;
            let mut to_move = Vec::new();
            let mut indexes = Vec::new();
            for x in bad_numbers_map.get(num).unwrap().iter() {
                let Some(pos) = ordered_manual.iter().position(|y| x == y) else {
                    continue;
                };
                indexes.push(pos);
            }
            indexes.sort();
            for (min, pos) in indexes.into_iter().enumerate() {
                to_move.push(ordered_manual.remove(pos - min));
            }
            ordered_manual.push(*num);
            ordered_manual.append(&mut to_move);
            rules.add_error_vals_by_key(*num, &mut bad_numbers_map);
        }
        if was_bad {
            let middle_number = *ordered_manual.get(self.0.len() / 2).unwrap();
            return Some(middle_number);
        }
        None
    }
}

fn parse_input(input: &str) -> IResult<&str, (Rules, Vec<Manual>)> {
    let rule = terminated(
        separated_pair(complete::u64, tag("|"), complete::u64),
        newline,
    );
    let manual = terminated(separated_list1(tag(","), complete::u64), opt(newline));
    let (rest, (rules, manuals)) = many_till(rule, preceded(newline, many1(manual)))(input)?;
    Ok((
        rest,
        (
            Rules::new(rules),
            manuals
                .into_iter()
                .map(|x| Manual::new(x))
                .collect::<Vec<Manual>>(),
        ),
    ))
}

pub fn part1(file: String) -> u64 {
    let (_leftovers, (rules, manuals)) = parse_input(file.as_str()).unwrap();
    manuals.iter().filter_map(|x| x.is_good(&rules)).sum()
}

pub fn part2(file: String) -> u64 {
    let (_leftovers, (rules, manuals)) = parse_input(file.as_str()).unwrap();
    manuals.iter().filter_map(|x| x.self_fixing(&rules)).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 143)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST.to_owned()), 123)
    }
}
