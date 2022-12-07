use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("couldn't read input");
    let assignment_pairs: Vec<&str> = input.lines().collect();
    println!("{}", part1(&assignment_pairs));
    println!("{}", part2(&assignment_pairs));
}

fn part1(input: &Vec<&str>) -> i32 {
    let mut count = 0;

    for pair in input {
        let (assignment1, assignment2) = pair.split_once(',').unwrap();
        let ((x1, y1), (x2, y2)) = (
            assignment1.split_once('-').unwrap(),
            assignment2.split_once('-').unwrap(),
        );
        if (x2.parse::<i32>().unwrap() >= x1.parse::<i32>().unwrap()
            && y2.parse::<i32>().unwrap() <= y1.parse::<i32>().unwrap())
            || (x2.parse::<i32>().unwrap() <= x1.parse::<i32>().unwrap()
                && y2.parse::<i32>().unwrap() >= y1.parse::<i32>().unwrap())
        {
            count += 1;
        }
    }
    count
}

fn part2(input: &Vec<&str>) -> i32 {
    let mut count = 0;
    for pair in input {
        let (assignment1, assignment2) = pair.split_once(',').unwrap();
        let (pair1, pair2) = (
            assignment1.split_once('-').unwrap(),
            assignment2.split_once('-').unwrap(),
        );
        let ((x1, y1), (x2, y2)) = (
            (
                pair1.0.parse::<i32>().unwrap(),
                pair1.1.parse::<i32>().unwrap(),
            ),
            (
                pair2.0.parse::<i32>().unwrap(),
                pair2.1.parse::<i32>().unwrap(),
            ),
        );

        if x1 <= y2 && x2 <= y1 {
            count += 1;
        }
    }
    count
}
