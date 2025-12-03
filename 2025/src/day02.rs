use std::collections::HashSet;

#[inline(always)]
fn digits_num(n: u64) -> u64 {
    (n as f64).log10().floor() as u64 + 1
}

fn check_group(v: u64, group_size: u64) -> bool {
    let digits = digits_num(v);
    if digits % group_size != 0 {
        return false;
    }

    let mask = 10_u64.pow(group_size as u32);
    let base = v % mask;

    let mut n = 0;
    for _ in 0..digits/group_size {
        n *= mask;
        n += base
    }

    n == v
}

fn check(v: u64) -> bool {
    let digits = digits_num(v);

    for group_size in 1..=digits/2 {
        if check_group(v, group_size) {
            return true;
        }
    }

    false
}

fn main() {
    let input = include_str!("input02")
        .trim()
        .split(',')
        .map(|range| {
            let (s,e) = range.split_once('-').unwrap();
            let s: u64 = s.parse().unwrap();
            let e: u64 = e.parse().unwrap();
            s..=e
        })
        .collect::<Vec<_>>();


    let part1 = input.iter()
        .cloned()
        .flat_map(|range| range)
        .filter(|&v| {
            let digits = digits_num(v);
            digits % 2 == 0 && check_group(v, digits / 2)
        })
        .sum::<u64>();

    println!("part1: {}", part1);

    let mut seen = HashSet::new();
    let part2 = input.iter()
        .cloned()
        .flat_map(|range| range)
        .filter(|&v| check(v))
        .filter(|id| seen.insert(*id)) // dedups by value
        .sum::<u64>();

    println!("part2: {}", part2);
}
