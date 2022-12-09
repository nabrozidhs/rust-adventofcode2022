use std::collections::HashMap;

fn _day07(input: &str) -> HashMap<String, (Vec<u64>, Vec<String>)> {
    let mut directories: HashMap<String, (Vec<u64>, Vec<String>)> = HashMap::new();
    let mut current_directory: Vec<&str> = vec![];
    for line in input.lines() {
        if line.starts_with("$ ls") {
            continue;
        }
        if line == "$ cd .." {
            current_directory.pop();
            continue;
        }
        if line.starts_with("$ cd") {
            current_directory.push(line.split(" ").last().unwrap());
            if !directories.contains_key(&current_directory.join("/")) {
                directories.insert(current_directory.join("/"), (vec![], vec![]));
            }
            continue;
        }
        if line.starts_with("dir") {
            directories.get_mut(&current_directory.join("/")).unwrap().1.push(
                current_directory.join("/") + "/" + line.split(" ").last().unwrap(),
            );
            continue;
        }
        let mut split = line.split(" ");
        directories.get_mut(&current_directory.join("/")).unwrap().0.push(
            split.next().unwrap().parse().unwrap(),
        );
    }
    directories
}

fn _size(tree: &HashMap<String, (Vec<u64>, Vec<String>)>, dir: &String) -> u64 {
    let v = tree.get(dir).unwrap();
    let file_size: u64 = v.0.iter().sum();
    file_size + v.1.iter().map(|f| _size(tree, f)).sum::<u64>()
}

fn _day07_part1(input: &str) -> u64 {
    let tree = _day07(input);
    tree.keys().map(|folder| {
        let size = _size(&tree, folder);
        if size <= 100000 {
            size
        } else {
            0
        }
    }).sum()
}

fn _day07_part2(input: &str) -> u64 {
    let tree = _day07(input);
    let unused = 70000000 - _size(&tree, &"/".to_string());
    let needed = 30000000 - unused;
    tree.keys().map(|folder| _size(&tree, folder))
        .filter(|&size| size >= needed)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests_day07 {
    use std::fs::File;
    use std::io::Read;

    use crate::day07::{_day07_part1, _day07_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(95437, _day07_part1("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(24933642, _day07_part2("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k"));
    }

    #[test]
    fn day07() {
        let mut file = File::open("data/day07.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(1306611, _day07_part1(&buffer));
        assert_eq!(13210366, _day07_part2(&buffer));
    }
}
