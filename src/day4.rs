use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use glam::IVec2;

pub fn run(part: u8) {
    let file = File::open("src/inputs/4.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
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

pub fn part1(file: String) -> usize {
    let locations = file
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
pub fn part2(file: String) -> usize {
    let locations = file
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
    const TEST: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    const TEST2: &str = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 18)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST2.to_owned()), 9)
    }
}
