use std::{collections::HashMap};

fn main() {
    let pts = HashMap::from([
        ("A", 1), // rock
        ("B", 2), // paper
        ("C", 3), // scissors
        ("X", 1), // rock
        ("Y", 2), // paper
        ("Z", 3), // scissors
    ]);

    let result = HashMap::from([
        (("A", "Z"), 0), (("B", "X"), 0), (("C", "Y"), 0),
        (("A", "X"), 3), (("B", "Y"), 3), (("C", "Z"), 3), 
        (("A", "Y"), 6), (("B", "Z"), 6), (("C", "X"), 6), 
    ]);

    let data: Vec<(&str, &str)> = include_str!("./input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .collect();

    let score = data
        .iter()
        .fold(0, |acc, x| acc + pts[x.1] + result[&x]);

    println!("part1 {}", score);


    let winning = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);
    let losing = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);

    let score2 = data
        .iter()
        .fold(0, |acc, x| {
            match x.1 {
                "X" => acc + pts[losing[x.0]] + 0,
                "Y" => acc + pts[x.0] + 3,
                "Z" => acc + pts[winning[x.0]] + 6,
                _ => unreachable!()
            }
        });

    println!("part2 {}", score2);
    // another idea is to operate on numbers instead of letters
}
