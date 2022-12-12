#![allow(dead_code)]

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: String,
    divisible_by: u64,
    on_true_throw_to: usize,
    on_false_throw_to: usize,
}

impl Monkey {
    fn operation(&self, old: u64) -> u64 {
        let op: Vec<&str> = self.operation.split(" ").collect();
        if op.last().unwrap() == &"old" {
            old * old
        } else {
            let num = op.last().unwrap().parse::<u64>().unwrap();
            if op[1] == "+" {
                old + num
            } else {
                old * num
            }
        }
    }
}

fn _parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];
    for group in input.split("\n\n") {
        let mut lines = group.lines();
        // skip 1
        lines.next();

        let items: Vec<u64> = lines.next().unwrap()
            .split(": ").last().unwrap()
            .split(", ").map(|f| f.parse::<u64>().unwrap())
            .collect();
        let operation: String = lines.next().unwrap().split(" = ").last().unwrap().to_string();
        let divisible_by = lines.next().unwrap().split(" by ").last().unwrap().parse::<u64>().unwrap();
        let on_true_throw_to = lines.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
        let on_false_throw_to = lines.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
        monkeys.push(
            Monkey {
                items,
                operation,
                divisible_by,
                on_true_throw_to,
                on_false_throw_to,
            }
        )
    }

    monkeys
}

fn _day11(input: &str, part2: bool) -> usize {
    let mut monkeys = _parse(input);
    let mut monkey_inspections: HashMap<usize, usize> = HashMap::new();
    for i in 0..monkeys.len() {
        monkey_inspections.insert(i, 0);
    }

    let product = monkeys.iter().map(|m| m.divisible_by).fold(1, |acc, v| acc * v);
    for _ in 0..(if part2 { 10000 } else { 20 }) {
        for index in 0..monkeys.len() {
            while !monkeys[index].items.is_empty() {
                let item = monkeys[index].items.pop().unwrap();
                let new_item = if part2 {
                    monkeys[index].operation(item) % product
                } else {
                    monkeys[index].operation(item) / 3
                };
                if new_item % monkeys[index].divisible_by == 0 {
                    let throw_to = monkeys[index].on_true_throw_to;
                    monkeys[throw_to].items.push(new_item);
                } else {
                    let throw_to = monkeys[index].on_false_throw_to;
                    monkeys[throw_to].items.push(new_item);
                }
                let prev_inspections: usize = monkey_inspections[&index];
                monkey_inspections.insert(index, prev_inspections + 1);
            }
        }
    }

    monkey_inspections.values().sorted().rev().take(2).fold(1, |acc, v| acc * v)
}

fn _day11_part1(input: &str) -> usize {
    _day11(input, false)
}

fn _day11_part2(input: &str) -> usize {
    _day11(input, true)
}

#[cfg(test)]
mod tests_day11 {
    use std::fs::File;
    use std::io::Read;

    use crate::day11::{_day11_part1, _day11_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(10605, _day11_part1("Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(2713310158, _day11_part2("Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1"));
    }

    #[test]
    fn day11() {
        let mut file = File::open("data/day11.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(78960, _day11_part1(&buffer));
        assert_eq!(14561971968, _day11_part2(&buffer));
    }
}
