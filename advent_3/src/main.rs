use std::fs::read_to_string;

fn main() {
    let mut prio = 0;

    let mut priorizer = new_badge_priorizer();

    for line in read_to_string("./input.txt").unwrap().lines() {
        prio += priorizer.get_prio(line);
    }

    println!("{}", prio);
}

trait Priorizer {
    fn get_prio(&mut self, line: &str) -> u32;
}

struct BadgePriorizer {
    lines: [String; 2],
    lines_nb: usize,
}

fn new_badge_priorizer() -> BadgePriorizer {
    return BadgePriorizer {
        lines_nb: 0,
        lines: ["".to_string(), "".to_string()],
    };
}

impl Priorizer for BadgePriorizer {
    fn get_prio(&mut self, line: &str) -> u32 {
        if self.lines_nb < 2 {
            self.lines[self.lines_nb] = line.to_string();
            self.lines_nb += 1;
            return 0;
        }

        let common: char = line
            .chars()
            .into_iter()
            .filter(|c| self.lines[0].contains(*c))
            .filter(|c| self.lines[1].contains(*c))
            .next()
            .unwrap();

        self.lines_nb = 0;
        return to_prio(common).unwrap();
    }
}

struct ItemPriorizer {}

impl Priorizer for ItemPriorizer {
    fn get_prio(&mut self, line: &str) -> u32 {
        let mut chars: Vec<_> = line.chars().collect();
        let compartment1 = chars.split_off(chars.len() / 2);
        let compartment2 = chars;

        let common: Vec<char> = compartment1
            .into_iter()
            .filter(|c| compartment2.contains(c))
            .collect();

        if let Some(c) = common.first() {
            let local_prio = to_prio(c.clone()).unwrap();
            return local_prio;
        }
        return 0;
    }
}

fn to_prio(item_type: char) -> Option<u32> {
    match item_type {
        'a'..='z' => Some(item_type as u32 - 'a' as u32 + 1),
        'A'..='Z' => Some(item_type as u32 - 'A' as u32 + 27),
        _ => None, // return None for non-alphabetic characters
    }
}
