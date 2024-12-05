use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use glam::IVec2;

pub fn run(part: u8) {
    let file = File::open("src/inputs/4.txt").unwrap();
    let reader = BufReader::new(file);
    match part {
        1 => dbg!(part1(reader)),
        2 => dbg!(part2(reader)),
        _ => return,
    };
}

fn check_matches(starting_point: &IVec2, locations: &HashMap<IVec2, char>) -> usize {
    let directions = [
        [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
        [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
        [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
        [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
        [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
        [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
        [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
        [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)],
    ];
    directions
        .iter()
        .map(|single_direction| {
            single_direction
                .iter()
                .filter_map(|offset| {
                    let new_location = starting_point.wrapping_add(*offset);
                    locations.get(&new_location)
                })
                .collect::<String>()
        })
        .filter(|x| x == "MAS")
        .count()
}
fn check_matches_p2(starting_point: &IVec2, locations: &HashMap<IVec2, char>) -> bool {
    let directions = [
        [IVec2::new(1, 1), IVec2::new(-1, -1)],
        [IVec2::new(1, -1), IVec2::new(-1, 1)],
    ];
    directions
        .iter()
        .map(|single_direction| {
            single_direction
                .iter()
                .filter_map(|offset| {
                    let new_location = starting_point.wrapping_add(*offset);
                    locations.get(&new_location)
                })
                .collect::<String>()
        })
        .filter(|x| x == "MS" || x == "SM")
        .count()
        == 2
}

pub fn part1(mut file: BufReader<File>) -> usize {
    let mut whole_file = String::new();
    file.read_to_string(&mut whole_file).unwrap();
    let locations = whole_file
        .lines()
        .enumerate()
        .flat_map(|(y, text)| {
            text.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>();
    locations
        .iter()
        .filter(|(_location, value)| **value == 'X')
        .map(|(location, _)| check_matches(location, &locations))
        .sum()
}
pub fn part2(mut file: BufReader<File>) -> usize {
    let mut whole_file = String::new();
    file.read_to_string(&mut whole_file).unwrap();
    let locations = whole_file
        .lines()
        .enumerate()
        .flat_map(|(y, text)| {
            text.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>();
    locations
        .iter()
        .filter(|(_location, value)| **value == 'A')
        .map(|(location, _)| check_matches_p2(location, &locations))
        .filter(|x| *x == true)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_works() {
        let file = File::open("src/testinputs/4.txt").unwrap();
        let reader = BufReader::new(file);
        assert_eq!(part1(reader), 18)
    }
    #[test]
    fn part2_works() {
        let file = File::open("src/testinputs/4-2.txt").unwrap();
        let reader = BufReader::new(file);
        assert_eq!(part2(reader), 9)
    }
}
