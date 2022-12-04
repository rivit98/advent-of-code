fn main() {
    let data: Vec<((u64, u64), (u64, u64))> = include_str!("./input.txt")
    .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(l, r)| (l.split_once("-").unwrap(), r.split_once("-").unwrap()))
        .map(|((l1, r1), (l2, r2))| ((l1.parse().unwrap(), r1.parse().unwrap()), (l2.parse().unwrap(), r2.parse().unwrap())))
        .collect();

    let part1 = data
        .iter()
        .filter(|((l1, r1), (l2, r2))| (l1 <= l2 && r2 <= r1) || (l2 <= l1 && r1 <= r2))
        .count();

    println!("part1 {}", part1);

    let part2 = data
        .iter()
        .filter(|((l1, r1), (l2, r2))| l1 <= r2 && l2 <= r1)
        .count();

    println!("part2 {}", part2);
}

// [l1; r1] [l2; r2]
// 


