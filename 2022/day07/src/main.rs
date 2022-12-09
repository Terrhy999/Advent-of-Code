use std::fs;

// $ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k

// [ (/, 14848514+8504156 + (29116+2557+62596+584)) ]
// total = 584 + (29116+2557+62596+584)
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut stack: Vec<(&str, usize)> = vec![];
    let mut total = 0;

    for line in input.lines() {
        if line.starts_with("$ cd") {
            if line.starts_with("$ cd ..") {
                let (_, amount) = stack.pop().unwrap();
                if amount <= 100000 {
                    total += amount;
                }
                stack.last_mut().unwrap().1 += amount;
            } else {
                let dir_name = &line[..5];
                stack.push((dir_name, 0));
                continue;
            }
        } else {
            let (amount, _) = line.split_once(" ").unwrap();

            if let Ok(amount) = amount.parse::<usize>() {
                stack.last_mut().unwrap().1 += amount;
            }
        }
    }
    println!("Total: {}", total);
}

fn part2(input: &String) {
    const TOTAL_SPACE: usize = 70000000;
    const SPACE_NEEDED: usize = 30000000;

    let mut stack: Vec<(&str, usize)> = vec![];
    let mut directories: Vec<(&str, usize)> = vec![];

    for line in input.lines() {
        if line.starts_with("$ cd") {
            if line.starts_with("$ cd ..") {
                let (dir_name, amount) = stack.pop().unwrap();
                directories.push((dir_name, amount));
                stack.last_mut().unwrap().1 += amount;
            } else {
                let dir_name = &line[5..];
                stack.push((dir_name, 0));
                continue;
            }
        } else {
            let (amount, _) = line.split_once(" ").unwrap();

            if let Ok(amount) = amount.parse::<usize>() {
                stack.last_mut().unwrap().1 += amount;
            }
        }
    }

    while !stack.is_empty() {
        let (name, amount) = stack.pop().unwrap();
        directories.push((name, amount));
        if !stack.is_empty() {
            stack.last_mut().unwrap().1 += amount;
        }
    }

    // for dir in &directories {
    //     println!("{}, {}", dir.0, dir.1)
    // }

    let unused_space = TOTAL_SPACE - directories.last().unwrap().1;
    // println!("Unused Space: {}", unused_space);
    let minimum_directory_size = SPACE_NEEDED - unused_space;
    // println!("min dir size: {}", minimum_directory_size);

    let smallest_valid_directory = directories
        .iter()
        .filter(|directory| directory.1 >= minimum_directory_size)
        .map(|dir| dir.1)
        .min()
        .unwrap();

    println!("Smallest valid directory: {:?}", smallest_valid_directory);
}
