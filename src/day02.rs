#![allow(dead_code)]

#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn from_string(value: &str) -> Self {
        match value {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!()
        }
    }

    pub fn hand_score(&self) -> i64 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    pub fn wins(&self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn round_score(&self, other: &Hand) -> i64 {
        if self == other {
            3
        } else if self.wins() == *other {
            6
        } else {
            0
        }
    }
}

fn _day02(input: &str, f: &dyn Fn(&str) -> i64) -> i64 {
    input.split("\n\n")
        .map(|group| group.lines().map(f).sum::<i64>())
        .sum()
}

fn _day02_part1(input: &str) -> i64 {
    _day02(input, &|input| {
        let mut split = input.split_whitespace();
        let left = Hand::from_string(split.next().unwrap());
        let right = Hand::from_string(
            match split.next().unwrap() {
                "X" => "A",
                "Y" => "B",
                "Z" => "C",
                _ => panic!()
            },
        );
        right.hand_score() + right.round_score(&left)
    })
}

fn _day02_part2(input: &str) -> i64 {
    _day02(input, &|input| {
        let mut split = input.split_whitespace();
        let left = Hand::from_string(split.next().unwrap());
        let right = match split.next().unwrap() {
            // draw
            "Y" => left,
            // lose
            "X" => left.wins(),
            // wins
            "Z" => left.loses_to(),
            _ => panic!(),
        };

        right.hand_score() + right.round_score(&left)
    })
}

#[cfg(test)]
mod tests_day02 {
    use std::fs::File;
    use std::io::Read;

    use crate::day02::{_day02_part1, _day02_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(15, _day02_part1("A Y\nB X\nC Z"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(12, _day02_part2("A Y\nB X\nC Z"));
    }

    #[test]
    fn day02() {
        let mut file = File::open("data/day02.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(12740, _day02_part1(&buffer));
        assert_eq!(11980, _day02_part2(&buffer));
    }
}
