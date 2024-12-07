use std::{fs::File, io::BufReader};

pub fn run(part: u8) {
    let file = File::open("src/inputs/1.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut whole_file = String::new();
    reader.read_to_string(whole_file).unwrap();
    match part {
        1 => dbg!(part1(whole_file)),
        2 => dbg!(part2(whole_file)),
        _ => return,
    };
}
pub fn part1(file: String) -> usize {
    0
}
pub fn part2(file: String) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST: &str = r#""#;
    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST.to_owned()), 0)
    }
    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST.to_owned()), 0)
    }
}
