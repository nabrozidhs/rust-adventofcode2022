use std::collections::HashSet;

use lazy_static::lazy_static;

lazy_static! {
    static ref CYCLES_TO_CHECK: HashSet < i64 > = HashSet::from([20, 60, 100, 140, 180, 220]);
}

fn _check_cycle(cycle: i64, x: i64, draw: bool) -> i64 {
    let draw_cycle = cycle - 1;
    if draw {
        if draw_cycle % 40 == 0 {
            println!();
        }
        print!(
            "{}", if (x - 1..=x + 1).contains(&(draw_cycle % 40)) { "#" } else { "." }
        );
    }
    if CYCLES_TO_CHECK.contains(&cycle) {
        x * cycle
    } else {
        0
    }
}

fn _day10(input: &str, draw: bool) -> i64 {
    let mut x = 1;
    let mut cycle = 1;
    let mut strength = 0;
    for line in input.lines() {
        let mut splitted = line.split(" ");
        let cmd = splitted.next().unwrap();
        strength += _check_cycle(cycle, x, draw);
        match cmd {
            "noop" => {
                cycle += 1;
            }
            "addx" => {
                cycle += 1;
                strength += _check_cycle(cycle, x, draw);
                cycle += 1;
                x += splitted.next().unwrap().parse::<i64>().unwrap();
            }
            _ => panic!(),
        }
    }
    strength
}

fn _day10_part1(input: &str) -> i64 {
    _day10(input, false)
}

fn _day10_part2(input: &str) -> i64 {
    _day10(input, true)
}

#[cfg(test)]
mod tests_day10 {
    use std::fs::File;
    use std::io::Read;

    use crate::day10::{_day10_part1, _day10_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(13140, _day10_part1("addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop"));
    }

    #[test]
    fn part2_test_input() {
        // _day10_part2("addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop");
    }

    #[test]
    fn day10() {
        let mut file = File::open("data/day10.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(11780, _day10_part1(&buffer));
        _day10_part2(&buffer);
    }
}
