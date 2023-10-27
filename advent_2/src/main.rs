use std::fs::read_to_string;

fn main() {
    let score = read_to_string("./input.txt")
        .unwrap()
        .lines()
        .into_iter()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .filter(|letters| letters.len() == 2)
        .map(|letters| {
            play(
                letters.get(0).expect("should be letter"),
                letters.get(1).expect("should be letter"),
            )
        })
        .reduce(|acc, score| acc + score);

    println!("{}", score.expect("should be score"));
}

fn play(a: &str, b: &str) -> u32 {
    match (a, b) {
        ("A", "Y") => 4,
        ("A", "X") => 3,
        ("A", "Z") => 8,
        ("B", "Y") => 5,
        ("B", "X") => 1,
        ("B", "Z") => 9,
        ("C", "Z") => 7,
        ("C", "X") => 2,
        ("C", "Y") => 6,
        (_, _) => 0,
    }
}
