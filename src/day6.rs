use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Read},
};

use glam::IVec2;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn run(part: u8) {
    let file = File::open("src/inputs/6.txt").unwrap();
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

fn rotate(offset: &mut IVec2) {
    match (offset.x, offset.y) {
        (0, -1) => {
            offset.x = 1;
            offset.y = 0;
        }
        (1, 0) => {
            offset.x = 0;
            offset.y = 1;
        }
        (0, 1) => {
            offset.x = -1;
            offset.y = 0;
        }

        (-1, 0) => {
            offset.x = 0;
            offset.y = -1;
        }
        _ => {}
    }
}

pub fn part1(file: String) -> usize {
    let mut map = parse_input(file);
    let starting_point = map
        .iter()
        .filter(|x| *x.1 == '^')
        .map(|x| x.0)
        .collect::<Vec<&IVec2>>()[0];
    let mut current_position = starting_point.clone();
    let mut offset = IVec2::new(0, -1);
    *map.get_mut(&current_position).unwrap() = 'X';
    loop {
        let next_position = current_position.wrapping_add(offset);
        let Some(val) = map.get_mut(&next_position) else {
            break;
        };
        match *val {
            '.' => {
                *val = 'X';
                current_position = next_position;
            }
            'X' => current_position = next_position,
            '#' => {
                rotate(&mut offset);
            }
            _ => {}
        }
    }
    map.values().filter(|x| **x == 'X').count()
}

fn walk_with_new_obstacle(
    new_obstacle: &IVec2,
    starting_point: &IVec2,
    map: &HashMap<IVec2, char>,
) -> usize {
    let mut offset = IVec2::new(0, -1);
    let mut current_position = starting_point.clone();
    let mut new_path = vec![*starting_point];
    loop {
        let current_index = new_path
            .iter()
            .position(|x| *x == current_position)
            .unwrap();
        let next_position = current_position.wrapping_add(offset);
        if let Some(next_index) = new_path.get(current_index + 1) {
            if next_position == *next_index {
                return 1;
            }
        }
        if next_position == *new_obstacle {
            rotate(&mut offset);
            continue;
        }
        let Some(val) = map.get(&next_position) else {
            break;
        };
        match *val {
            '.' => {
                new_path.push(next_position);
                current_position = next_position;
            }
            '^' => {
                new_path.push(next_position);
                current_position = next_position;
            }
            '#' => {
                rotate(&mut offset);
            }
            _ => {}
        }
    }
    0
}
pub fn part2(file: String) -> usize {
    let map = parse_input(file);
    let starting_point = map
        .iter()
        .filter(|x| *x.1 == '^')
        .map(|x| x.0)
        .collect::<Vec<&IVec2>>()[0];
    let mut originally_walked_on = HashSet::new();
    let mut current_position = starting_point.clone();
    let mut offset = IVec2::new(0, -1);
    originally_walked_on.insert(starting_point.clone());
    loop {
        let next_position = current_position.wrapping_add(offset);
        let Some(val) = map.get(&next_position) else {
            break;
        };
        match *val {
            '.' => {
                originally_walked_on.insert(next_position.clone());
                current_position = next_position;
            }
            '^' => {
                originally_walked_on.insert(next_position.clone());
                current_position = next_position;
            }
            '#' => {
                rotate(&mut offset);
            }
            _ => {}
        }
    }
    originally_walked_on
        .par_iter()
        .map(|x| walk_with_new_obstacle(x, &starting_point, &map))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 41)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST.to_owned()), 6)
    }
}
