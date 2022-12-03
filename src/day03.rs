use std::collections::HashSet;

fn _char_to_priority(c: &char) -> i64 {
    let c_cast = *c as i64;
    if c_cast as i64 >= 97 {
        c_cast as i64 - 96
    } else {
        c_cast as i64 - 38
    }
}

fn _day03_part1(input: &str) -> i64 {
    input.lines()
        .map(|line| {
            let mut left: HashSet<char> = HashSet::new();
            let mut right: HashSet<char> = HashSet::new();
            line.chars().enumerate().for_each(|(i, c)| {
                if i < line.len() / 2 {
                    left.insert(c);
                } else {
                    right.insert(c);
                }
            });
            let character = *left.intersection(&right).next().unwrap();
            _char_to_priority(&character)
        })
        .sum()
}

fn _day03_part2(input: &str) -> i64 {
    let mut items: HashSet<char> = HashSet::new();
    input.lines().enumerate()
        .map(|(index, line)| {
            let mut new_items: HashSet<char> = HashSet::new();
            line.chars().for_each(|c| {
                new_items.insert(c);
            });

            if index % 3 == 0 {
                items = new_items;
            } else {
                items = items.intersection(&new_items)
                    .cloned()
                    .collect();
            }

            if index % 3 == 2 {
                let character = *items.iter().next().unwrap();
                _char_to_priority(&character)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests_day03 {
    use std::fs::File;
    use std::io::Read;

    use crate::day03::{_day03_part1, _day03_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(157, _day03_part1("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(70, _day03_part2("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"));
    }

    #[test]
    fn day03() {
        let mut file = File::open("data/day03.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(8053, _day03_part1(&buffer));
        assert_eq!(2425, _day03_part2(&buffer));
    }
}
