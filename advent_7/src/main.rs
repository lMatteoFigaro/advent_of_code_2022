use std::fs::read_to_string;

fn main() {
    for line in read_to_string("./input.txt").unwrap().lines() {
        println!("{}", line);
    }
}

enum FileSystem {
    File,
    Dir,
}

struct Dir {
    childs: Vec<FileSystem>,
    size: u64,
    name: String,
}

struct File {
    size: u64,
    name: String,
}
