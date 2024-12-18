use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

use glam::IVec2;

static OFFSETS: [IVec2; 4] = [
    IVec2::new(0, 1),
    IVec2::new(1, 0),
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
];

pub fn run(part: u8) {
    let file = File::open("src/inputs/12.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(&mut whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
        _ => return,
    };
}

fn follow_region(
    current_plot: (&IVec2, &Plot),
    plot_size: &mut u64,
    perimeter: &mut u64,
    area: &HashMap<IVec2, Plot>,
) {
    *current_plot.1.checked.borrow_mut() = true;
    OFFSETS
        .iter()
        .map(|x| area.get_key_value(&(x + current_plot.0)))
        .for_each(|x| {
            let Some(x) = x else {
                *perimeter += 1;
                return;
            };
            if x.1.value != current_plot.1.value {
                *perimeter += 1;
                return;
            }
            if *x.1.checked.borrow() {
                return;
            }
            *plot_size += 1;
            follow_region(x, plot_size, perimeter, area);
        });
}

fn follow_region_p2(
    current_plot: (&IVec2, &Plot),
    plot_size: &mut u64,
    perimeter: &mut HashMap<IVec2, Vec<Edge>>,
    area: &HashMap<IVec2, Plot>,
) {
    *current_plot.1.checked.borrow_mut() = true;
    OFFSETS.iter().for_each(|os| {
        let maybe = os + current_plot.0;
        let edge = Edge::new(*os, RefCell::new(false));
        let Some(x) = area.get_key_value(&maybe) else {
            perimeter
                .entry(*current_plot.0)
                .and_modify(|x| x.push(edge.clone()))
                .or_insert(vec![edge]);
            return;
        };
        if x.1.value != current_plot.1.value {
            perimeter
                .entry(*current_plot.0)
                .and_modify(|x| x.push(edge.clone()))
                .or_insert(vec![edge]);
            return;
        }
        if *x.1.checked.borrow() {
            return;
        }
        *plot_size += 1;
        follow_region_p2(x, plot_size, perimeter, area);
    });
}

fn calculate_sides(perimeter: &mut HashMap<IVec2, Vec<Edge>>) -> usize {
    let mut sides = 0;
    loop {
        let Some(location) = perimeter
            .iter()
            .find(|x| x.1.iter().any(|y| !*y.checked.borrow()))
        else {
            break;
        };
        let edge = location.1.iter().find(|x| !*x.checked.borrow()).unwrap();
        println!("Starting at {} on edge {}", location.0, edge.value);
        sides += count_turns(perimeter, *location.0, edge.value, 0);
    }
    println!("Total sides: {sides}");
    sides
}

#[derive(Clone, PartialEq, Eq)]
struct Edge {
    value: IVec2,
    checked: RefCell<bool>,
}

impl Edge {
    fn new(value: IVec2, checked: RefCell<bool>) -> Self {
        Self { value, checked }
    }
}

fn count_turns(
    perimeter: &HashMap<IVec2, Vec<Edge>>,
    location: IVec2,
    edge: IVec2,
    current: usize,
) -> usize {
    *perimeter
        .get(&location)
        .unwrap()
        .iter()
        .find(|x| x.value == edge)
        .unwrap()
        .checked
        .borrow_mut() = true;
    let current_direction = edge.perp();

    //check straightaway
    let maybe_next_location = location + current_direction;
    if let Some(adj_plot) = perimeter.get_key_value(&maybe_next_location) {
        if let Some(edge) = adj_plot
            .1
            .iter()
            .find(|x| x.value == edge && !*x.checked.borrow())
        {
            return count_turns(perimeter, *adj_plot.0, edge.value, current);
        }
    }
    // check right turn on same location
    if let Some(thing) = perimeter
        .get(&location)
        .unwrap()
        .iter()
        .find(|x| x.value == current_direction)
    {
        if !*thing.checked.borrow() {
            return count_turns(perimeter, location, current_direction, current + 1);
        } else {
            return current + 1;
        }
    }
    // check left turn, catty corner location
    let maybe_catty_corner = maybe_next_location + edge;
    let opposite_edge = -1 * current_direction;
    if let Some(catty_corner) = perimeter.get_key_value(&maybe_catty_corner) {
        if let Some(e) = catty_corner.1.iter().find(|x| x.value == opposite_edge) {
            if !*e.checked.borrow() {
                return count_turns(perimeter, maybe_catty_corner, opposite_edge, current + 1);
            } else {
                return current + 1;
            }
        }
    };
    current
}

pub fn part1(file: String) -> u64 {
    let area = parse_input(file);
    area.iter()
        .filter_map(|x| {
            if *x.1.checked.borrow() {
                return None;
            }
            let mut plot_size = 1;
            let mut perimeter = 0;
            follow_region(x, &mut plot_size, &mut perimeter, &area);
            Some(plot_size * perimeter)
        })
        .sum()
}
pub fn part2(file: String) -> u64 {
    let area = parse_input(file);
    area.iter()
        .filter_map(|x| {
            if *x.1.checked.borrow() {
                return None;
            }
            let mut plot_size = 1;
            let mut perimeter = HashMap::new();
            println!("following region for {} starting at {}", x.1.value, x.0);
            follow_region_p2(x, &mut plot_size, &mut perimeter, &area);
            println!("Value: {}, plot_size: {}", &x.1.value, &plot_size);
            Some(plot_size * calculate_sides(&mut perimeter) as u64)
        })
        .sum()
}

#[derive(Debug)]
struct Plot {
    value: char,
    checked: RefCell<bool>,
}

impl Plot {
    fn new(value: char) -> Self {
        Self {
            value,
            checked: RefCell::new(false),
        }
    }
}

fn parse_input(input: String) -> HashMap<IVec2, Plot> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, text)| {
            text.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), Plot::new(value)))
        })
        .collect::<HashMap<IVec2, Plot>>()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 1930)
    }
    #[test]
    fn part2_works() {
        println!("testing part 2");
        assert_eq!(part2(TEST.to_owned()), 1206)
    }
}
