use std::collections::BinaryHeap;
use std::fs::read_to_string;

fn main() {
    let mut elves = BinaryHeap::new();

    let mut current_cal = 0;
    for line in read_to_string("./input.txt").unwrap().lines() {
        if line == "" {
            elves.push(current_cal);
            current_cal = 0;
            continue;
        }

        let cal: i32 = match line.parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        current_cal += cal;
    }

    elves.push(current_cal);

    println!(
        "{}",
        elves.pop().unwrap() + elves.pop().unwrap() + elves.pop().unwrap()
    )
}
