use std::collections::{HashSet};

struct Card {
    before: HashSet<usize>,
    after: HashSet<usize>,
}

// cache would be nice, but who cares
fn count(wins: &Vec<usize>, idx: usize) -> usize {
    if idx >= wins.len() {
        return 0;
    }

    let v: usize = (1..=wins[idx])
        .map(|widx| count(wins, idx+widx))
        .sum();

    return v + wins[idx];
}

fn main() {
    let data: Vec<_> = include_str!("./input")
        .lines()
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            let values_before = before.split_once(':').unwrap().1;

            return Card {
                after: after.split_ascii_whitespace().map(|token| token.parse::<usize>().unwrap()).collect(),
                before: values_before.split_ascii_whitespace().map(|token| token.parse::<usize>().unwrap()).collect()
            }
        })
        .collect();

    let wins: Vec<usize> = data.iter()
        .map(|card| card.before.intersection(&card.after).count())
        .collect();

    let part1: u64 = wins.iter()
        .filter_map(|&size| {
            if size == 0 {None} else { Some(2_u64.pow((size - 1) as u32)) }
        })
        .sum();

    println!("{part1}");

    let part2: usize = wins.iter()
        .enumerate()
        .filter(|(_, &size)| size > 0)
        .map(|(idx, _)| count(&wins, idx) )
        .sum();

    let part2 = part2 + wins.len();
    println!("{part2}");
}