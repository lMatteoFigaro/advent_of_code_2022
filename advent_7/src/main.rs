use regex::Regex;
use std::{borrow::BorrowMut, fs::read_to_string};

fn main() {
    let re = Regex::new(r"^- (.*) \(dir\)$").unwrap();
    let re_file = Regex::new(r"^- (.*) \(file, size=(\d+)\)$").unwrap();

    let mut parents = Vec::new();
    let mut current_dir = Option::<Dir>::None;
    let mut last_dir = Option::<Dir>::None;
    let space_count = 0;
    for line in read_to_string("./input.txt").unwrap().lines() {
        let count = count_spaces_before_char(line, '-');

        let line = line.trim();

        if let Some(caps) = re.captures(line) {
            if let Some(name) = caps.get(1) {
                println!("is dir with name {}", name.as_str());
                let dir = Dir {
                    childs: Vec::new(),
                    size: 0,
                    name: name.as_str().to_string(),
                };
                if let Some(dir) = current_dir.borrow_mut() {
                    parents.push(dir);
                }
                last_dir = Option::Some(dir)
            }
        }

        if let Some(caps) = re_file.captures(line) {
            if let (Some(name), Some(size)) = (caps.get(1), caps.get(2)) {
                println!(
                    "is file with size {} and name {}",
                    size.as_str(),
                    name.as_str()
                );
                let file = File {
                    size: size.as_str().parse().unwrap(),
                    name: name.as_str().to_string(),
                };
                if let Some(dir) = current_dir.borrow_mut() {
                    dir.childs.push(FileSystem::File(file.clone()));
                    dir.size = dir.size + file.size;
                }
            }
        }
    }
}

enum FileSystem {
    File(File),
    Dir,
}

struct Dir {
    childs: Vec<FileSystem>,
    size: u64,
    name: String,
}

#[derive(Clone)]
struct File {
    size: u64,
    name: String,
}

fn count_spaces_before_char(s: &str, target: char) -> usize {
    let mut space_count = 0;

    for c in s.chars() {
        if c == target {
            return space_count;
        } else if c == ' ' {
            space_count += 1;
        } else {
            space_count = 0;
        }
    }
    space_count
}
