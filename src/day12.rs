#![allow(dead_code)]
use std::collections::HashSet;
use crate::util::Vector2;

#[derive(Debug)]
struct Board {
    heightmap: Vec<Vec<i64>>,
    start: Vector2,
    end: Vector2,
}

impl Board {
    fn new(input: &str) -> Board {
        let mut heightmap: Vec<Vec<i64>> = vec![];
        let mut end: Vector2 = Vector2::ZERO;
        let mut start: Vector2 = Vector2::ZERO;

        for (y, line) in input.lines().enumerate() {
            let mut row: Vec<i64> = vec![];
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        start = Vector2::new(x as i64, y as i64);
                        row.push(0);
                    }
                    'E' => {
                        end = Vector2::new(x as i64, y as i64);
                        row.push('z' as i64 - 97);
                    }
                    _ => row.push(c as i64 - 97),
                }
            }
            heightmap.push(row);
        }

        Board { heightmap, start, end }
    }
}

fn _find_path(board: &Board, part2: bool) -> Vec<Vector2> {
    let mut queue: Vec<Vec<Vector2>> = vec![vec![board.end.clone()]];
    let mut visited: HashSet<Vector2> = HashSet::from([board.end.clone()]);
    loop {
        let path = queue.remove(0);
        let last_point = *path.last().unwrap();
        let last_height = board.heightmap[last_point.1 as usize][last_point.0 as usize];
        let next_points = vec![
            last_point + Vector2(1, 0),
            last_point + Vector2(-1, 0),
            last_point + Vector2(0, 1),
            last_point + Vector2(0, -1),
        ];
        for next_point in next_points {
            if next_point.0 < 0 || next_point.0 >= board.heightmap[0].len() as i64 ||
                next_point.1 < 0 || next_point.1 >= board.heightmap.len() as i64 ||
                visited.contains(&next_point) {
                continue;
            }

            let next_height = board.heightmap[next_point.1 as usize][next_point.0 as usize];
            if (0..=(next_height + 1)).contains(&last_height) {
                let mut new_path = path.clone();
                new_path.push(next_point);
                if (part2 && next_height == 0) || next_point == board.start {
                    return new_path;
                }
                visited.insert(next_point);
                queue.push(new_path);
            }
        }
    }
}

fn _day12_part1(input: &str) -> usize {
    _find_path(&Board::new(input), false).len() - 1
}

fn _day12_part2(input: &str) -> usize {
    _find_path(&Board::new(input), true).len() - 1
}

#[cfg(test)]
mod tests_day12 {
    use std::fs::File;
    use std::io::Read;

    use crate::day12::{_day12_part1, _day12_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(31, _day12_part1("Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(29, _day12_part2("Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi"));
    }

    #[test]
    fn day12() {
        let mut file = File::open("data/day12.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(528, _day12_part1(&buffer));
        assert_eq!(522, _day12_part2(&buffer));
    }
}
