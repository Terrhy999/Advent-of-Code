use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &String) -> i32 {
    let rounds = input.lines().collect::<Vec<&str>>();

    let mut total = 0;

    for round in rounds {
        let result = match round {
            "B X" => 1,
            "C Y" => 2,
            "A Z" => 3,
            "A X" => 4,
            "B Y" => 5,
            "C Z" => 6,
            "C X" => 7,
            "A Y" => 8,
            "B Z" => 9,
            _ => 0,
        };

        total += result;
    }

    total
}

fn part2(input: &String) -> i32 {
    let rounds = input.lines().collect::<Vec<&str>>();

    let mut total = 0;

    for round in rounds {
        let result = match round {
            "B X" => 1,
            "C X" => 2,
            "A X" => 3,
            "A Y" => 4,
            "B Y" => 5,
            "C Y" => 6,
            "C Z" => 7,
            "A Z" => 8,
            "B Z" => 9,
            _ => 0,
        };

        total += result;
    }
    total
}
