use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() {
    let mut stacks = Stacks::new();

    let re = regex::Regex::new("move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    for line in read_to_string("./input.txt").unwrap().lines() {
        if line == "" {
            println!("finished stack {:?}", stacks);
            stacks.print();
            println!("");
            continue;
        }

        if let Some(caps) = re.captures(line) {
            println!("{}", line);
            let repeat: u8 = caps[1].parse().unwrap();
            let from: usize = caps[2].parse().unwrap();
            let to: usize = caps[3].parse().unwrap();
            stacks.move_elem_stack(repeat, from, to);
        };

        if !line.contains(|c| c == '[' || c == ']') {
            continue;
        }

        line.chars().enumerate().for_each(|(i, c)| {
            if i == 0 {
                return;
            }
            let pos = i - 1;
            if pos % 4 == 0 && c != ' ' {
                stacks.queue(pos / 4, c)
            }
        });
    }

    println!("{:?}", stacks);
    stacks.print();
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

    fn move_elem(&mut self, num: u8, from: usize, to: usize) {
        let mut i = num;
        let from = from - 1;
        let to = to - 1;
        while i > 0 {
            let Some(elem) = self
                .stacks
                .get_mut(from)
                .expect("could not get stack")
                .pop_front()
            else {
                return;
            };
            self.stacks.get_mut(to).unwrap().push_front(elem);
            i = i - 1;
        }
    }

    fn move_elem_stack(&mut self, num: u8, from: usize, to: usize) {
        let mut i = num;
        let from = from - 1;
        let to = to - 1;

        let mut current_stack = VecDeque::new();
        while i > 0 {
            let Some(elem) = self
                .stacks
                .get_mut(from)
                .expect("could not get stack")
                .pop_front()
            else {
                return;
            };
            current_stack.push_back(elem);
            i = i - 1;
        }

        println!("got stack {:?}", current_stack);

        while let Some(elem) = current_stack.pop_back() {
            self.stacks.get_mut(to).unwrap().push_front(elem);
        }
    }

    fn print(&self) {
        for e in &self.stacks {
            print!("{}", e.front().unwrap());
        }
    }
}
