use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("couldn't read input");
    let assignment_pairs: Vec<&str> = input.lines().collect();
    println!("{}", part1(assignment_pairs));
}

fn part1(input: Vec<&str>) -> i32 {
    let mut count = 0;

    for pair in input {
        let (assignment1, assignment2) = pair.split_once(',').unwrap();
        let ((x, y), (a, b)) = (
            assignment1.split_once('-').unwrap(),
            assignment2.split_once('-').unwrap(),
        );
        if (a.parse::<i32>().unwrap() >= x.parse::<i32>().unwrap()
            && b.parse::<i32>().unwrap() <= y.parse::<i32>().unwrap())
            || (a.parse::<i32>().unwrap() <= x.parse::<i32>().unwrap()
                && b.parse::<i32>().unwrap() >= y.parse::<i32>().unwrap())
        {
            count += 1;
        }
    }
    count
}
