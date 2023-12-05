use std::cmp::{min};
use std::ops::Range;
use std::thread;

fn main() {
    let data: Vec<&str>  =include_str!("./t")
        .split("\n\n")
        .collect();

    let seeds: Vec<u64> = data[0].splitn(2, |c| c == ':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|token| token.parse::<u64>().unwrap())
        .collect();

    let stages: Vec<Vec<(Range<u64>, Range<u64>, u64)>> = data[1..].iter()
        .map(|group| {
            group.lines()
                .skip(1)
                .map(|line| {
                    let range: Vec<u64> = line.split_ascii_whitespace().map(|token| token.parse::<u64>().unwrap()).collect();
                    (range[0]..range[0]+range[2], range[1]..range[1]+range[2], range[2])
                })
                .collect()
        })
        .collect();



    let current_stages: Vec<(Range<u64>, u64)> = vec![];
    for stage in stages {
        for (dest, src, len) in stage {
            println!("{dest:?}");
            println!("{src:?}");
        }
    }

}