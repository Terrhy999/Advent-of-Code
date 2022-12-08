use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("parsing input.txt to string");
    let chars: Vec<char> = input[..].chars().collect();
    println!("{}", find_start_of_message(&chars, 4));
    println!("{}", find_start_of_message(&chars, 14))
}

fn find_start_of_message(input: &Vec<char>, window_size: usize) -> usize {
    let windows = input.windows(window_size);

    let mut message_start = 0;
    for (index, window) in windows.enumerate() {
        if window.iter().all_unique() {
            message_start = index + window_size;
            break;
        }
    }
    message_start
}
