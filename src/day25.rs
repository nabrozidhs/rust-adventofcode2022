fn _snafu_to_decimal(snafu: &str) -> i64 {
    snafu.chars().rev().enumerate()
        .map(|(index, c)| {
            let v = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => panic!(),
            } as i64;
            v * 5_i64.pow(index as u32)
        })
        .sum()
}

fn _decimal_to_snafu(decimal: i64) -> String {
    let mut v = decimal;
    let mut snafu = String::new();
    while v > 0 {
        snafu.insert(
            0,
            match v % 5 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '=',
                4 => '-',
                _ => panic!(),
            },
        );
        v = (v + 2) / 5;
    }
    snafu
}

fn _day25(input: &str) -> String {
    let sum = input.lines().map(_snafu_to_decimal).sum();
    _decimal_to_snafu(sum)
}

#[cfg(test)]
mod tests_day25 {
    use std::fs::File;
    use std::io::Read;

    use crate::day25::_day25;

    #[test]
    fn test_input() {
        assert_eq!(
            String::from("2=-1=0"),
            _day25("1=-0-2\n12111\n2=0=\n21\n2=01\n111\n20012\n112\n1=-1=\n1-12\n12\n1=\n122"),
        );
    }

    #[test]
    fn day25() {
        let mut file = File::open("data/day25.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(String::from(""), _day25(&buffer));
    }
}
