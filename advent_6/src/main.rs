use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let reader = read_to_string("./input.txt").unwrap();
    let line = reader.lines().next().unwrap();

    let mut window = VecDeque::new();
    line.chars().into_iter().enumerate().for_each(|(i, c)| {
        if window.len() < 13 && !window.contains(&c) {
            window.push_back(c);
            return;
        }

        while let Some(elem) = window.pop_front() {
            println!("at char {} pos {}, popped char {}", c, i, elem);
            if elem == c {
                window.push_back(c);
                return;
            }
        }

        println!("{}", i + 1);
        std::process::exit(0)
    })
}
