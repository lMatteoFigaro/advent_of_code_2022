use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() {
    let mut stacks = Stacks::new();
    for line in read_to_string("./input.txt").unwrap().lines() {
        if line == "" || !line.contains(|c| c == '[' || c == ']') {
            break;
        }

        line.replace('[', "")
            .replace(']', "")
            .replace("   ", " ")
            .chars()
            .enumerate()
            .for_each(|(i, c)| {
                print!("{}", c);
                if c != ' ' {
                    stacks.queue(i / 2, c)
                }
            });

        println!("");
    }

    println!("{:?}", stacks);
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<VecDeque<char>>,
}

impl Stacks {
    fn new() -> Stacks {
        return Stacks { stacks: Vec::new() };
    }

    fn queue(&mut self, i: usize, val: char) {
        while self.stacks.len() < i + 1 {
            self.stacks.push(VecDeque::new());
        }

        self.stacks.get_mut(i).unwrap().push_back(val)
    }
}
