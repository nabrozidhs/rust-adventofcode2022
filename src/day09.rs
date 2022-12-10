use std::collections::HashSet;

use crate::util::Vector2;

fn _day09(input: &str, rope_length: usize) -> usize {
    let mut visited = HashSet::new();
    let mut rope: Vec<Vector2> = vec![];
    for _ in 0..rope_length {
        rope.push(Vector2::ZERO);
    }
    visited.insert(rope.last().unwrap().clone());
    for line in input.lines() {
        let mut splitted = line.split(" ");
        let heading = match splitted.next().unwrap() {
            "R" => Vector2(1, 0),
            "L" => Vector2(-1, 0),
            "U" => Vector2(0, 1),
            "D" => Vector2(0, -1),
            _ => panic!(),
        };
        for _ in 0..splitted.next().unwrap().parse::<usize>().unwrap() {
            let mut new_rope: Vec<Vector2> = vec![];
            new_rope.push(rope[0].clone() + heading);
            for i in 1..rope.len() {
                let new_heading = match rope[i] - new_rope[i - 1] {
                    Vector2(-2, -2) => Vector2(-1, -1),
                    Vector2(-2, 2) => Vector2(-1, 1),
                    Vector2(2, -2) => Vector2(1, -1),
                    Vector2(2, 2) => Vector2(1, 1),
                    Vector2(2, _) => Vector2(1, 0),
                    Vector2(-2, _) => Vector2(-1, 0),
                    Vector2(_, 2) => Vector2(0, 1),
                    Vector2(_, -2) => Vector2(0, -1),
                    v => v,
                };
                new_rope.push(new_rope[i - 1] + new_heading);
            }

            rope = new_rope;
            visited.insert(rope.last().unwrap().clone());
        }
    }
    visited.len()
}

fn _day09_part1(input: &str) -> usize {
    _day09(input, 2)
}

fn _day09_part2(input: &str) -> usize {
    _day09(input, 10)
}

#[cfg(test)]
mod tests_day09 {
    use std::fs::File;
    use std::io::Read;

    use crate::day09::{_day09_part1, _day09_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(13, _day09_part1("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(1, _day09_part2("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"));
        assert_eq!(36, _day09_part2("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20"));
    }

    #[test]
    fn day09() {
        let mut file = File::open("data/day09.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(5710, _day09_part1(&buffer));
        assert_eq!(2259, _day09_part2(&buffer));
    }
}
