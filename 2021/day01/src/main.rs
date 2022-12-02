use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").expect("Cannot open .txt file");
    part1(&input);
    part2(&input);
}

fn part1(input: &File) {
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

fn part2(input: &File) {
    let input = File::open("input.txt").expect("Cannot open .txt file");
    let reader = BufReader::new(input);

    let count = reader
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect::<Vec<u16>>()
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .collect::<Vec<u16>>();
    // .windows(3)
    // .filter(|y| y[0] < y[1])
    // .count();

    for num in count {
        println!("{}", num);
    }
}
