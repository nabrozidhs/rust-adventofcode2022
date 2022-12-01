use itertools::Itertools;

fn _day01(input: &str) -> impl Iterator<Item=i64> + '_ {
    input.split("\n\n")
        .map(|group| group.split("\n").map(|x| x.parse::<i64>().unwrap()).sum())
}

fn _day01_part1(input: &str) -> i64 {
    _day01(input).max().unwrap()
}

fn _day01_part2(input: &str) -> i64 {
    _day01(input)
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests_day01 {
    use std::fs::File;
    use std::io::Read;

    use crate::day01::{_day01_part1, _day01_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(24000, _day01_part1("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(45000, _day01_part2("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"));
    }

    #[test]
    fn day01() {
        let mut file = File::open("data/day01.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(69912, _day01_part1(&buffer));
        assert_eq!(208180, _day01_part2(&buffer));
    }
}
