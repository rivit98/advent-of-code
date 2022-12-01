use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


use itertools::Itertools;

fn main() {
    let lines = read_lines("./input.txt");
    let groups = lines.iter()
        .group_by(|l| (**l).len() == 0)
        .into_iter()
        .filter(|(ge0, _)| *ge0 == false)
        .map(|(_, group)| group.cloned().map(|line| line.parse::<u64>().unwrap()).collect())
        .collect::<Vec<Vec<u64>>>();

    let sums: Vec<u64> = groups.iter()
        .map(|g| g.iter().sum())
        .collect();

    println!("part1 {}", sums.iter().max().unwrap());
    println!("part2 {}", sums.iter().sorted().rev().take(3).sum::<u64>())

}

fn read_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    reader.lines()
        .map(|l| l.unwrap())
        .collect()
}