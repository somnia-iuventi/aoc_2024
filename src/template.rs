use std::{fs::File, io::BufReader};

pub fn run(part: u8) {
    let file = File::open("src/inputs/1.txt").unwrap();
    let mut reader = BufReader::new(file);
    match part {
        1 => part1(reader),
        2 => part2(reader),
        _ => return,
    };
}
pub fn part1(mut file: BufReader<File>) -> usize {
    0
}
pub fn part2(mut file: BufReader<File>) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_works() {
        let file = File::open("src/testinputs/1.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert_eq!(part1(reader), 0)
    }
    #[test]
    fn part2_works() {
        let file = File::open("src/testinputs/1.txt").unwrap();
        let mut reader = BufReader::new(file);
        assert_eq!(part2(reader), 0)
    }
}
