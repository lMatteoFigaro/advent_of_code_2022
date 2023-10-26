use std::{io::{BufReader, BufRead}, fs::File};

fn main() {
    let mut count = 0;

    for line in BufReader::new(File::open("./input.txt").unwrap()).lines() {
        let line = line.unwrap();
        println!("{}", line);

        let intervals: Vec<EInterval> = line.split(',').map(|s| {
                let interval: Vec<String> = s.split('-').map(|s|s.to_string()).collect();

                return EInterval{start: interval.get(0).unwrap().parse().unwrap(), end: interval.get(1).unwrap().parse().unwrap()}
        }).collect();

        let int1 = intervals.get(0).unwrap();
        let int2 = intervals.get(1).unwrap();

        if overlap(int1, int2) {
            count += 1;
        }
    }
    println!("count: {}", count);
}

fn overlap(int1: &EInterval, int2: &EInterval) -> bool {
    if contained(int1, int2) {
        return true 
    }

    return is_between(int1.start, int2.start, int2.end) ||
        is_between(int1.end, int2.start, int2.end) ||
        is_between(int2.start, int1.start, int1.end) ||
        is_between(int2.end, int1.start, int1.end)
}

fn is_between(a :usize, b:usize, c:usize) -> bool {
    return a >= b && a <= c 
}

fn contained(int1: &EInterval, int2: &EInterval) -> bool {
    if int1.start >= int2.start && int1.end <= int2.end {
        return true;
    }

    if int2.start >= int1.start && int2.end <= int1.end{
        return true;
    }

    return false;
}

#[derive(Debug)]
struct EInterval {
    start :usize,
    end :usize,
}


