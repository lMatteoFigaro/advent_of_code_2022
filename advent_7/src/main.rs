use std::{fs::read_to_string, thread::current};

fn main() {
    let mut root = Dir {
        childs: Vec::new(),
        size: 0,
        name: "/".to_string(),
    };

    let mut path = Vec::new();
    let mut current = &mut root;

    for line in read_to_string("./input.txt").unwrap().lines() {
        println!("{}", line);
        if line.starts_with("$ cd") {
            let name = line.split_whitespace().last().unwrap();
            if name == "/" {
                continue;
            }
            if name == ".." {
                path.pop();
                let previous_size = current.size;
                current = get_dir(&mut root, &path);
                current.size += previous_size;
            } else {
                path.push(name.to_string());
                current = get_dir(&mut root, &path);
            }
            println!("curent dir is {}", current.name)
        } else if line.starts_with("dir") {
            let elems: Vec<&str> = line.split(' ').collect();
            let name = elems.get(1).unwrap();

            current.childs.push(Dir {
                childs: Vec::new(),
                size: 0,
                name: name.to_string(),
            });
        } else if let Some(size) = starts_with_number(line) {
            current.size = current.size + size;
        }
    }
    println!("{:?}", root);

    let root_size = root.childs.iter().fold(0, |acc, dir| dir.size + acc);

    let sum = find_dir(root.clone()).iter().fold(0, |acc, dir| {
        if dir.size < 100000 {
            acc + dir.size
        } else {
            acc
        }
    });

    println!("{}", sum);

    const MAX_SIZE: u64 = 70000000;
    const MIN_AVAILABLE: u64 = 30000000;

    let available = MAX_SIZE - root_size;
    let required = MIN_AVAILABLE - available;

    let size = find_dir(root.clone())
        .iter()
        .filter(|dir| dir.size > required)
        .map(|dir| dir.size)
        .min()
        .unwrap();

    println!("min size to delete {}", size);
}

#[derive(Debug, Clone)]
struct Dir {
    childs: Vec<Dir>,
    size: u64,
    name: String,
}

#[derive(Clone)]
struct File {
    size: u64,
    name: String,
}

fn starts_with_number(s: &str) -> Option<u64> {
    s.split_whitespace()
        .next()
        .and_then(|first_part| first_part.parse::<u64>().ok())
}

fn get_dir<'a>(root: &'a mut Dir, path: &Vec<String>) -> &'a mut Dir {
    let mut current = root;
    if path.len() == 0 {
        return current;
    }

    for name in path {
        current = current
            .childs
            .iter_mut()
            .find(|dir| &dir.name == name)
            .unwrap()
    }

    return current;
}

fn find_dir(root: Dir) -> Vec<Dir> {
    let mut dirs = Vec::new();
    for dir in root.clone().childs {
        dirs.extend(find_dir(dir))
    }

    dirs.push(root);
    return dirs;
}
