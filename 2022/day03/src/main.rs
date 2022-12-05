use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("couldn't read the file");
    let lines: Vec<&str> = input.lines().collect();
    println!("{}", part1(lines));
}

fn part1(input: Vec<&str>) -> i32 {
    let backpacks: Vec<i32> = input
        .iter()
        .map(|backpack| {
            let (container1, container2) = backpack.split_at(backpack.len() / 2);
            let mut common_char = ' ';
            for item in container1.chars() {
                if container2.contains(item) {
                    common_char = item;
                }
            }
            item_to_score(common_char)
        })
        .collect();
    backpacks.iter().sum()
}

fn item_to_score(item: char) -> i32 {
    match item {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}
