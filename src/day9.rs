use std::{
    cmp::Ordering,
    collections::LinkedList,
    fs::File,
    io::{BufReader, Read},
};

use itertools::Itertools;

pub fn run(part: u8) {
    let file = File::open("src/inputs/9.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
        _ => return,
    };
}

#[derive(Debug)]
struct Space {
    id: i64,
    length: usize,
    tried: bool,
}

impl Space {
    fn new(id: i64, length: usize, tried: bool) -> Self {
        Self { id, length, tried }
    }
}

fn parse_input(input: String) -> LinkedList<Space> {
    let mut id = 0;
    input
        .chars()
        .enumerate()
        .filter_map(|x| {
            let file = x.0 % 2 == 0;
            let Ok(length) = x.1.to_string().parse::<usize>() else {
                return None;
            };
            if file {
                let ret = Space::new(id, length, false);
                id += 1;
                Some(ret)
            } else {
                Some(Space::new(-1, length, false))
            }
        })
        .collect::<LinkedList<Space>>()
}

pub fn part1(file: String) -> usize {
    let mut parsed = parse_input(file);
    let mut final_index = 0;
    loop {
        let Some(first_empty_index) = parsed
            .iter()
            .enumerate()
            .find(|x| x.1.id == -1)
            .map(|x| x.0)
        else {
            break;
        };
        let mut last = parsed.pop_back().unwrap();
        if last.id == -1 {
            continue;
        }
        let mut after = parsed.split_off(first_empty_index);
        let mut first_empty_space = after.pop_front().unwrap();
        match last.length.cmp(&first_empty_space.length) {
            Ordering::Less => {
                first_empty_space.length = first_empty_space.length - last.length;
                parsed.push_back(last);
                after.push_front(first_empty_space);
                parsed.append(&mut after);
            }
            Ordering::Equal => {
                parsed.push_back(last);
                parsed.append(&mut after);
            }
            Ordering::Greater => {
                let left_over_length = last.length - first_empty_space.length;
                let left_over = Space::new(last.id, left_over_length, false);
                last.length = first_empty_space.length;
                parsed.push_back(last);
                after.push_back(left_over);
                parsed.append(&mut after);
            }
        }
    }
    parsed
        .iter()
        .map(|x| {
            (0..x.length)
                .into_iter()
                .map(|_| {
                    let thing = x.id as usize * final_index;
                    final_index += 1;
                    thing
                })
                .sum::<usize>()
        })
        .sum()
}
pub fn part2(file: String) -> usize {
    let mut parsed = parse_input(file);
    let mut final_index = 0;
    loop {
        let full_list = parsed
            .iter()
            .enumerate()
            .rev()
            .filter(|x| x.1.id > -1 && !x.1.tried)
            .map(|x| (x.0, x.1.length))
            .collect_vec();

        let Some((full_ind, full_length)) = full_list.get(0) else {
            break;
        };

        let available_empty_spaces = parsed
            .iter()
            .enumerate()
            .filter(|x| x.1.id == -1 && x.1.length >= *full_length && x.0 < *full_ind)
            .map(|x| x.0)
            .collect_vec();
        let mut tail = parsed.split_off(*full_ind);
        let mut full = tail.pop_front().unwrap();
        let Some(empty_ind) = available_empty_spaces.get(0) else {
            full.tried = true;
            parsed.push_back(full);
            parsed.append(&mut tail);
            continue;
        };
        let new_empty = Space::new(-2, full.length, false);
        let mut mid = parsed.split_off(*empty_ind);
        let mut empty = mid.pop_front().unwrap();
        empty.length -= full.length;
        parsed.push_back(full);
        parsed.push_back(empty);
        tail.push_front(new_empty);
        parsed.append(&mut mid);
        parsed.append(&mut tail);
    }
    parsed
        .iter()
        .map(|x| {
            (0..x.length)
                .into_iter()
                .map(|_| {
                    if x.id < 0 {
                        final_index += 1;
                        return 0;
                    }
                    let answer = x.id as usize * final_index;
                    final_index += 1;
                    answer
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = "2333133121414131402";
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 1928)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST.to_owned()), 2858)
    }
}
