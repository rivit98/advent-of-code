#![feature(array_chunks)]
use std::collections::HashSet;

fn main() {
    let signals: Vec<char> = include_str!("./input.txt").trim().chars().collect();

    let calc = |chunk_size| {
        return signals.windows(chunk_size)
        .position(|chunk| {
            let s: HashSet<_> = HashSet::from_iter(chunk);
            s.len() == chunk.len()
        })
        .unwrap()
        + chunk_size;
    };
    
    println!("part1 {}", calc(4));
    println!("part2 {}", calc(14));
}


