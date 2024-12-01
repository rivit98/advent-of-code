use std::collections::HashMap;

fn main() {
    let data: Vec<(i64, i64)> = include_str!("./input.txt")
        .trim()
        .lines()
        .map(|line| {
            let mut iter = line
                .split_whitespace()
                .take(2)
                .map(|x| x.parse().unwrap());

            let left = iter.next().unwrap();
            let right = iter.next().unwrap();
            (left, right)
            }
        )
        .collect();

    let mut left: Vec<i64> = data.iter().map(|(first, _)| *first).collect();
    let mut right: Vec<i64> = data.iter().map(|(_, second)| *second).collect();
    left.sort();
    right.sort();

    let part1: u64 = left.iter().zip(right.iter()).map(|(a, b)| (a-b).abs() as u64).sum();
    println!("part 1: {part1}");

    let occurrences_right = right
        .iter()
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
    
    let part2: u64 = left.iter().map(|x| (occurrences_right.get(x).unwrap_or(&0) * x) as u64).sum();
    println!("part 2: {part2}");
}