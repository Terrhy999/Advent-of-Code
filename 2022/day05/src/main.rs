use std::{fs, str};

struct Input {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

#[derive(Clone)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("reading input.txt");
    let (input1, input2) = input
        .split_once("\n\n")
        .expect("split input into stacks and instructions");

    let (rows, num_row) = input1
        .rsplit_once("\n")
        .expect("split stacks from number row");

    let num_stacks: usize = num_row.split_whitespace().last().unwrap().parse().unwrap();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    let rows_reversed: Vec<&str> = rows.lines().rev().collect();

    for row in rows_reversed {
        let chunks: Vec<&str> = row
            .as_bytes()
            .chunks(4)
            .map(|byte| str::from_utf8(byte).expect("convert 4 bytes to string slice"))
            .collect();

        for (index, chunk) in chunks.iter().enumerate() {
            let crate_letter = chunk
                .chars()
                .nth(1)
                .expect("get the 2nd character in the chunk");

            if crate_letter.is_alphabetic() {
                stacks[index].push(crate_letter);
            };
        }
    }

    let instructions: Vec<Instruction> = input2
        .lines()
        .map(|line| {
            let instruction: Vec<&str> = line.split_whitespace().collect();

            Instruction {
                amount: instruction[1].parse().unwrap(),
                from: instruction[3].parse().unwrap(),
                to: instruction[5].parse().unwrap(),
            }
        })
        .collect();

    let part1_solution = part1(Input {
        stacks: stacks.clone(),
        instructions: instructions.clone(),
    });
    for stack in part1_solution {
        println!("{:?}", stack)
    }

    let part2_solution = part2(Input {
        stacks,
        instructions,
    });
    for stack in part2_solution {
        println!("{:?}", stack)
    }
}

fn part1(input: Input) -> Vec<Vec<char>> {
    let mut stacks = input.stacks.clone();

    for Instruction { amount, to, from } in input.instructions {
        for _ in 0..amount {
            let top_crate = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(top_crate);
        }
    }
    stacks
}

fn part2(input: Input) -> Vec<Vec<char>> {
    let mut stacks = input.stacks.clone();

    for Instruction { amount, to, from } in input.instructions {
        let stack_length = stacks[from - 1].len();
        let removed_crates = stacks[from - 1].split_off(stack_length - amount);
        stacks[to - 1].extend(removed_crates);
    }
    stacks
}
