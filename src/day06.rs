use std::collections::HashSet;

fn _day06(input: &str, previous: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut i = previous;
    loop {
        let mut set: HashSet<char> = HashSet::new();
        for k in (i-previous)..i {
            set.insert(chars[k]);
        }
        let diff = previous - set.len();
        if diff == 0 {
            return i
        }
        i += diff
    }
}

fn _day06_part1(input: &str) -> usize {
    _day06(input, 4)
}

fn _day06_part2(input: &str) -> usize {
    _day06(input, 14)
}

#[cfg(test)]
mod tests_day06 {
    use std::fs::File;
    use std::io::Read;

    use crate::day06::{_day06_part1, _day06_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(5, _day06_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, _day06_part1("nppdvjthqldpwncqszvftbrmjlhg"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(19, _day06_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, _day06_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    }

    #[test]
    fn day06() {
        let mut file = File::open("data/day06.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(1909, _day06_part1(&buffer.lines().next().unwrap()));
        assert_eq!(3380, _day06_part2(&buffer.lines().next().unwrap()));
    }
}
