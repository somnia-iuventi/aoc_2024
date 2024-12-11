use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use glam::IVec2;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

static OFFSETS: [IVec2; 4] = [
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
];

pub fn run(part: u8) {
    let file = File::open("src/inputs/10.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
        _ => return,
    };
}

fn parse_input(input: String) -> HashMap<IVec2, u8> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, text)| {
            text.chars().enumerate().map(move |(x, value)| {
                (
                    IVec2::new(x as i32, y as i32),
                    value.to_string().parse::<u8>().unwrap(),
                )
            })
        })
        .collect::<HashMap<IVec2, u8>>()
}

fn go_next(location: &IVec2, value: u8, topo: &HashMap<IVec2, u8>) -> Vec<IVec2> {
    let next_value = value + 1;
    OFFSETS
        .iter()
        .filter_map(|os| {
            let next_location = location + os;
            let Some(adj_val) = topo.get(&next_location) else {
                return None;
            };
            if *adj_val != next_value {
                return None;
            }
            if next_value == 9 {
                return Some(vec![next_location]);
            } else {
                return Some(go_next(&next_location, next_value, topo));
            }
        })
        .flatten()
        .unique()
        .collect_vec()
}

fn go_next_p2(location: &IVec2, value: u8, topo: &HashMap<IVec2, u8>) -> usize {
    let next_value = value + 1;
    OFFSETS
        .iter()
        .map(|os| {
            let next_location = location + os;
            let Some(adj_val) = topo.get(&next_location) else {
                return 0;
            };
            if *adj_val != next_value {
                return 0;
            }
            if next_value == 9 {
                return 1;
            } else {
                return go_next_p2(&next_location, next_value, topo);
            }
        })
        .sum()
}

pub fn part1(file: String) -> usize {
    let topo = parse_input(file);
    topo.par_iter()
        .filter(|x| *x.1 == 0)
        .map(|x| go_next(x.0, *x.1, &topo))
        .flatten()
        .count()
}
pub fn part2(file: String) -> usize {
    let topo = parse_input(file);
    topo.par_iter()
        .filter(|x| *x.1 == 0)
        .map(|x| go_next_p2(x.0, *x.1, &topo))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 36)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST.to_owned()), 81)
    }
}
