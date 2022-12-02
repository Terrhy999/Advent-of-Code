use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").expect("Cannot open .txt file");
    let reader = BufReader::new(input);

    let count = reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect::<Vec<u16>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();

    println!("{}", count);
}
