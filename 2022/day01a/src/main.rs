use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").expect("Couldn't read the file");
    let elfs_total_calories = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|meal| meal.parse::<i32>().unwrap()).sum())
        .collect::<Vec<i32>>();

    println!("{}", elfs_total_calories.iter().max().unwrap())
}
