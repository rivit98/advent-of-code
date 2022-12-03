use std::collections::HashSet;

fn main() {
    let rucksacks: Vec<&str> = include_str!("./input.txt").trim().lines().collect();

    let a: u64 = rucksacks
        .iter()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(l, r)| {
            (
                HashSet::<_>::from_iter(l.chars()),
                HashSet::<_>::from_iter(r.chars()),
            )
        })
        .map(|(l, r)| {
            l.intersection(&r)
                .cloned()
                .into_iter()
                .fold(0u64, |acc, v| match v {
                    'a'..='z' => acc + 1 + (v as u64) - ('a' as u64),
                    'A'..='Z' => acc + 27 + (v as u64) - ('A' as u64),
                    _ => unreachable!(),
                })
        })
        .sum();

    println!("part1 {}", a);

    let b: u64 = rucksacks
        .chunks_exact(3)
        .map(|chunk| {
            chunk
                .into_iter()
                .map(|l| HashSet::from_iter(l.chars()))
                .collect::<Vec<HashSet<char>>>()
        })
        .map(|groups| {
            groups
                .iter()
                .skip(1)
                .fold(groups[0].clone(), |acc, hs| {
                    acc.intersection(hs).cloned().collect()
                })
                .iter()
                .fold(0u64, |acc, v| {
                    match v {
                        'a'..='z' => acc + 1 + (*v as u64) - ('a' as u64),
                        'A'..='Z' => acc + 27 + (*v as u64) - ('A' as u64),
                        _ => unreachable!(),
                    }
                })
        })
        .sum();

    println!("part2 {}", b);
}
