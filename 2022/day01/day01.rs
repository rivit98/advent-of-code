fn main() {
    let mut sums: Vec<u64> = include_str!("./input.txt")
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse::<u64>().unwrap()).sum())
        .collect();

    sums.sort();

    println!("part1 {}", sums.iter().max().unwrap());
    println!("part2 {}", sums.iter().rev().take(3).sum::<u64>())
}
