#![allow(dead_code)]

use regex::Regex;

struct Cargo {
    stacks: Vec<Vec<char>>,
}

impl Cargo {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let num_stacks = (lines[0].len() + 1) / 4;
        let mut stacks: Vec<Vec<char>> = vec![];
        (0..num_stacks).for_each(|_| stacks.push(vec![]));

        for y in (0..(lines.len() - 1)).rev() {
            for x in 0..num_stacks {
                let value = lines[y].chars().collect::<Vec<char>>()[x * 4 + 1];
                if value != ' ' {
                    stacks[x].push(value);
                }
            }
        }

        Self {
            stacks,
        }
    }

    fn mv(&mut self, from: usize, to: usize, times: usize, multiple: bool) {
        for i in 0..times {
            let v: char = if multiple {
                let size = self.stacks[from].len();
                self.stacks[from].remove(size + i - times)
            } else {
                self.stacks[from].pop().unwrap()
            };
            self.stacks[to].push(v);
        }
    }
}

fn _day05(input: &str, multiple_move: bool) -> String {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut split = input.split("\n\n");
    let mut cargo = Cargo::new(split.next().unwrap());

    for line in split.next().unwrap().lines() {
        let cap = re.captures(line).unwrap();
        cargo.mv(
            cap[2].parse::<usize>().unwrap() - 1,
            cap[3].parse::<usize>().unwrap() - 1,
            cap[1].parse().unwrap(),
            multiple_move,
        );
    }

    cargo.stacks.into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

fn _day05_part1(input: &str) -> String {
    _day05(input, false)
}

fn _day05_part2(input: &str) -> String {
    _day05(input, true)
}

#[cfg(test)]
mod tests_day05 {
    use std::fs::File;
    use std::io::Read;

    use crate::day05::{_day05_part1, _day05_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!("CMZ", _day05_part1("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!("MCD", _day05_part2("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2"));
    }

    #[test]
    fn day05() {
        let mut file = File::open("data/day05.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!("BSDMQFLSP", _day05_part1(&buffer));
        assert_eq!("PGSQBFLDP", _day05_part2(&buffer));
    }
}
