//INPUT
// R 4
// U 4
// L 3
// D 1 BROKEN
// R 4 HERE?
// D 1
// L 5
// R 2

use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut board: HashSet<(i32, i32)> = std::collections::HashSet::new();

    for motion in input.lines() {
        let (dir, dis) = motion.split_once(" ").unwrap();

        for _ in 0..dis.parse::<u32>().unwrap() {
            match dir {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                _ => panic!("Unexpected direction"),
            }
            move_tail(&head, &mut tail, &mut board);
        }
        println!("Head: {:?}, Tail: {:?}", head, tail)
    }
    let unique_visited_spaces = board.iter().count();
    println!("{}", unique_visited_spaces);
}

fn move_tail(head: &(i32, i32), mut tail: &mut (i32, i32), board: &mut HashSet<(i32, i32)>) {
    let (delta_x, delta_y) = (head.0 - tail.0, head.1 - tail.1); // (2,-1)
                                                                 // println!("({},{})", delta_x, delta_y);

    if delta_x.abs() > 1 && delta_y.abs() == 0 {
        if delta_x.is_positive() {
            tail.0 += 1;
        } else {
            tail.0 -= 1;
        }
    }

    if delta_y.abs() > 1 && delta_x.abs() == 0 {
        if delta_y.is_positive() {
            tail.1 += 1;
        } else {
            tail.1 -= 1;
        }
    }

    if (delta_x.abs() > 1 && delta_y.abs() == 1) || delta_y.abs() > 1 && delta_x.abs() == 1 {
        if delta_x.is_positive() && delta_y.is_positive() {
            tail.0 += 1;
            tail.1 += 1;
        } else if delta_x.is_positive() && delta_y.is_negative() {
            tail.0 += 1;
            tail.1 -= 1;
        } else if delta_x.is_negative() && delta_y.is_positive() {
            tail.0 -= 1;
            tail.1 += 1;
        } else if delta_x.is_negative() && delta_y.is_negative() {
            tail.0 -= 1;
            tail.1 -= 1;
        }
    }
    board.insert(*tail);
}
