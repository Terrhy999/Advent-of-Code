use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("Couldn't read the file");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let elfs_total_calories = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|meal| meal.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    println!("{}", elfs_total_calories.iter().max().unwrap())
}

fn part2(input: &String) {
    let mut elfs_total_calories = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|meal| meal.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    elfs_total_calories.sort_unstable();
    elfs_total_calories.reverse();

    println!(
        "{}",
        elfs_total_calories[0] + elfs_total_calories[1] + elfs_total_calories[2]
    )
}
