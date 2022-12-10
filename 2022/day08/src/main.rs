use std::{collections::HashSet, fs};

// INPUT
// 30373
// 25512
// 65332
// 33549
// 35390

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let forest: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree| tree.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    part1(forest);
}

fn part1(forest: Vec<Vec<u32>>) {
    let number_of_columns = forest[0].len();
    let number_of_rows = forest.len();
    let trees_on_perimiter = number_of_columns * 2 + number_of_rows * 2 - 4;
    let mut tree_count = 0;

    for y in 1..number_of_rows - 1 {
        for x in 1..number_of_columns - 1 {
            //Is tree tallest from left
            let tree_height = forest[y][x];

            //From Left
            let visible_from_left = &tree_height > forest[y][..x].iter().max().unwrap();
            if visible_from_left {
                tree_count += 1;
                continue;
            }

            //From Right
            let visible_from_right = &tree_height > forest[y][x + 1..].iter().max().unwrap();
            if visible_from_right {
                tree_count += 1;
                continue;
            }

            //From Top
            let visible_from_top =
                &tree_height > &forest[..y].iter().map(|row| row[x]).max().unwrap();
            if visible_from_top {
                tree_count += 1;
                continue;
            }

            //From Bottom
            let visible_from_bottom =
                &tree_height > &forest[y + 1..].iter().map(|row| row[x]).max().unwrap();
            if visible_from_bottom {
                tree_count += 1;
                continue;
            }
        }
    }

    println!("{}", tree_count + trees_on_perimiter);
}
