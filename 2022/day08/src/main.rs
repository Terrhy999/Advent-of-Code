use std::fs;

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
    part1(&forest);
    part2(&forest);
}

fn part1(forest: &Vec<Vec<u32>>) {
    let number_of_columns = forest[0].len();
    let number_of_rows = forest.len();
    let trees_on_perimiter = number_of_columns * 2 + number_of_rows * 2 - 4;
    let mut tree_count = 0;

    for y in 1..number_of_rows - 1 {
        for x in 1..number_of_columns - 1 {
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

fn part2(forest: &Vec<Vec<u32>>) {
    let number_of_columns = forest[0].len();
    let number_of_rows = forest.len();
    let mut tree_scores: Vec<usize> = vec![];

    // forest = [
    // [3, 0, 3, 7, 3]
    // [2, 5, 5, 1, 2]
    // [6, 5, 3, 3, 2]
    // [3, 3, 5, 4, 9]
    // [3, 5, 3, 9, 0]
    // ]

    for y in 1..number_of_rows - 1 {
        for x in 1..number_of_columns - 1 {
            let tree_height = forest[y][x];
            let mut last = true;

            let visible_trees_on_left = &forest[y][..x]
                .iter()
                .rev()
                .take_while(|tree| {
                    last = tree < &&tree_height;
                    last
                })
                .count()
                + if last { 0 } else { 1 };

            //From Right
            let visible_trees_on_right = &forest[y][x + 1..]
                .iter()
                .take_while(|tree| {
                    last = tree < &&tree_height;
                    last
                })
                .count()
                + if last { 0 } else { 1 };

            // //From Top
            let visible_trees_on_top = &forest[..y]
                .iter()
                .map(|row| row[x])
                .rev()
                .take_while(|tree| {
                    last = tree < &&tree_height;
                    last
                })
                .count()
                + if last { 0 } else { 1 };

            // //From Bottom
            let visible_trees_on_bottom = &forest[y + 1..]
                .iter()
                .map(|row| row[x])
                .take_while(|tree| {
                    last = tree < &&tree_height;
                    last
                })
                .count()
                + if last { 0 } else { 1 };

            let score = visible_trees_on_left
                * visible_trees_on_right
                * visible_trees_on_top
                * visible_trees_on_bottom;

            tree_scores.push(score);
        }
    }
    println!("{}", tree_scores.iter().max().unwrap())
}
