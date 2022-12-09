use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("read input.txt to string");
    let lines = input.lines();

    let mut filesystem: Directory = Directory {
        files: HashMap::new(),
    };

    let cwd: Directory;

    
    for line in lines {
        if line.starts_with("$") {
            let command = line.split_whitespace().skip(1).next().expect("get the command after the '$ ");

            match command {
                "cd"
            }
        }
        
}

struct Directory {
    files: HashMap<String, Node>,
}

struct File {
    size: usize,
}

enum Node {
    Directory(Directory),
    File(File),
}
