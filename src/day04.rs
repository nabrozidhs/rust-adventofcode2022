fn _day04(input: &str, f: &dyn Fn(i64, i64, i64, i64) -> i64) -> i64 {
    input.lines()
        .map(|line| {
            let mut splitted = line.split(",");
            let mut left = splitted.next().unwrap().split("-");
            let mut right = splitted.next().unwrap().split("-");

            let l1: i64 = left.next().unwrap().parse().unwrap();
            let l2: i64 = left.next().unwrap().parse().unwrap();
            let r1: i64 = right.next().unwrap().parse().unwrap();
            let r2: i64 = right.next().unwrap().parse().unwrap();

            f(l1, l2, r1, r2)
        })
        .sum()
}

fn _day04_part1(input: &str) -> i64 {
    _day04(input, &|l1, l2, r1, r2| {
        if (l1 <= r1 && l2 >= r2) || (r1 <= l1 && r2 >= l2) {
            1
        } else {
            0
        }
    })
}

fn _day04_part2(input: &str) -> i64 {
    _day04(input, &|l1, l2, r1, r2| {
        if l1 > r2 || r1 > l2 {
            0
        } else {
            1
        }
    })
}

#[cfg(test)]
mod tests_day04 {
    use std::fs::File;
    use std::io::Read;

    use crate::day04::{_day04_part1, _day04_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(2, _day04_part1("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(4, _day04_part2("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"));
    }

    #[test]
    fn day04() {
        let mut file = File::open("data/day04.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(441, _day04_part1(&buffer));
        assert_eq!(861, _day04_part2(&buffer));
    }
}
