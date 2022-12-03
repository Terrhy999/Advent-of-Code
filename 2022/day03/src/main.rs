use array_tool::vec::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./input.txt").expect("couldn't read the file");
    let input = BufReader::new(file);
}

fn part1(input: BufReader<File>) {
    let lines = input.lines();
    let backpacks = lines.map(|line| line.unwrap());

    let compartments = backpacks.map(|backpack| {
        let compartment_size = backpack.len() / 2;
        let (compartment1, compartment2) = backpack.split_at(compartment_size);
        [compartment1, compartment2]
    });

    let compartments_intersection = compartments.map(|compartment| {
        let item_in_common = 
    })
}
