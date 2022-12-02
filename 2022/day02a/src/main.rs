use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

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

    println!("{}", total);
}
