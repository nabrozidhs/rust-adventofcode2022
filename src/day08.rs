fn _build_board(input: &str) -> Vec<Vec<u8>> {
    let mut board: Vec<Vec<u8>> = vec![];
    for line in input.lines() {
        let mut row: Vec<u8> = vec![];
        for c in line.chars() {
            row.push(c as u8 - 48);
        }
        board.push(row);
    }
    return board;
}

fn _visible_trees(board: &Vec<Vec<u8>>) -> usize {
    let mut visible = 0;
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if y == 0 || y == board.len() - 1 || x == 0 || x == board[0].len() - 1 {
                visible += 1;
            } else {
                let v = board[y][x];
                'inner: for heading in [(1 as i64, 0 as i64), (-1, 0), (0, 1), (0, -1)] {
                    let mut i = (x as i64 + heading.0, y as i64 + heading.1);
                    while i.0 >= 0 && i.0 <= (board[0].len() - 1) as i64 && i.1 >= 0 && i.1 <= (board.len() - 1) as i64 {
                        if v <= board[i.1 as usize][i.0 as usize] {
                            continue 'inner;
                        }
                        i = (i.0 + heading.0, i.1 + heading.1);
                    }
                    visible += 1;
                    break;
                }
            }
        }
    }
    visible
}

fn _max_scenic(board: &Vec<Vec<u8>>) -> usize {
    let mut max_scenic = 0;
    for y in 1..(board.len() - 1) {
        for x in 1..(board[0].len() - 1) {
            let mut scenic = 1;

            for heading in [(1 as i64, 0 as i64), (-1, 0), (0, 1), (0, -1)] {
                let mut i = (x as i64 + heading.0, y as i64 + heading.1);
                let mut num_visible = 0;
                while i.0 >= 0 && i.0 <= (board[0].len() - 1) as i64 && i.1 >= 0 && i.1 <= (board.len() - 1) as i64 {
                    let v = board[i.1 as usize][i.0 as usize];
                    if board[y][x] <= v {
                        num_visible += 1;
                        break;
                    }
                    num_visible += 1;
                    i = (i.0 + heading.0, i.1 + heading.1);
                }
                scenic = scenic * num_visible;
            }

            if scenic > max_scenic {
                max_scenic = scenic;
            }
        }
    }
    max_scenic
}

fn _day08_part1(input: &str) -> usize {
    let board = _build_board(input);
    _visible_trees(&board)
}

fn _day08_part2(input: &str) -> usize {
    let board = _build_board(input);
    _max_scenic(&board)
}

#[cfg(test)]
mod tests_day08 {
    use std::fs::File;
    use std::io::Read;

    use crate::day08::{_day08_part1, _day08_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(21, _day08_part1("30373\n25512\n65332\n33549\n35390"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(8, _day08_part2("30373\n25512\n65332\n33549\n35390"));
    }

    #[test]
    fn day08() {
        let mut file = File::open("data/day08.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(1711, _day08_part1(&buffer));
        assert_eq!(301392, _day08_part2(&buffer));
    }
}
