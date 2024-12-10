use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use glam::IVec2;
use itertools::Itertools;

pub fn run(part: u8) {
    let file = File::open("src/inputs/8.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
        _ => return,
    };
}

fn parse_input(input: String) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, text)| {
            text.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>()
}

fn find_antinode(location: &IVec2, value: &char, hmap: &HashMap<IVec2, char>) -> Vec<IVec2> {
    hmap.iter()
        .filter(|x| x.1 == value && x.0 != location)
        .map(|x| x.0)
        .filter_map(|x| {
            let double_distance = 2 * (x - location);
            hmap.get_key_value(&(*location + double_distance))
        })
        .map(|x| *x.0)
        .collect_vec()
}

fn find_antinode_p2(location: &IVec2, value: &char, hmap: &HashMap<IVec2, char>) -> Vec<IVec2> {
    hmap.iter()
        .filter(|x| x.1 == value && x.0 != location)
        .map(|x| x.0)
        .flat_map(|x| {
            let mut distance = *x - location;
            while distance.x % 2 == 0 && distance.y % 2 == 0 {
                distance = distance / 2;
            }
            let mut current_distance = distance.clone();
            let mut answers = Vec::new();
            while let Some(val) = hmap.get_key_value(&(*location + current_distance)) {
                answers.push(*val.0);
                current_distance += distance;
            }
            answers
        })
        .collect_vec()
}

pub fn part1(file: String) -> usize {
    let hmap = parse_input(file);
    hmap.iter()
        .filter(|x| *x.1 != '.')
        .flat_map(|x| find_antinode(x.0, x.1, &hmap))
        .unique()
        .count()
}
pub fn part2(file: String) -> usize {
    let hmap = parse_input(file);
    hmap.iter()
        .filter(|x| *x.1 != '.')
        .flat_map(|x| find_antinode_p2(&x.0, &x.1, &hmap))
        .unique()
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 14)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST.to_owned()), 34)
    }
}
