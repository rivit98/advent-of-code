use std::collections::{HashMap, HashSet};
use regex::{Regex};

#[derive(Eq, PartialEq, Hash, Clone)]
struct Number {
    x1: i32,
    x2: i32,
    y: i32,
    value: i32,
}

fn main() {
    let mut symbols: Vec<(i32, i32)> = vec![];
    let mut gears: Vec<(i32, i32)> = vec![];
    let re = Regex::new(r"(\d+)").unwrap();

    let data: HashMap<(i32, i32), Number> = include_str!("./input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .into_iter()
                .enumerate()
                .for_each(|(x, c)| {
                    if !c.is_digit(10) && c != '.' {
                        symbols.push((x as i32, y as i32))
                    }
                    if c == '*' {
                        gears.push((x as i32, y as i32))
                    }
                });

            re.captures_iter(line)
                .map(|c| c.get(1).unwrap())
                .map(|m| Number {
                    x1: m.start() as i32,
                    x2: (m.end() - 1) as i32,
                    y: y as i32,
                    value: m.as_str().parse().unwrap(),
                })
                .collect::<Vec<_>>()
        })
        .flat_map(|n|
            (n.x1..=n.x2).map(move |x| ((x, n.y), n.clone()))
        )
        .collect();

    let part1: i32 = symbols.iter()
        .flat_map(|&(x, y)|
            (-1..=1).flat_map(move |dx| (-1..=1).map(move |dy| (x + dx, y + dy)))
                .filter(move |&(nx, ny)| nx != x || ny != y)
        )
        .filter_map(|s| data.get(&s))
        .collect::<HashSet<_>>()
        .iter()
        .map(|n| n.value)
        .sum();

    println!("{part1}");

    let part2: i32 = gears.iter()
        .map(|&(x, y)| {
            let numbers: Vec<i32> = (-1..=1).flat_map(move |dx| (-1..=1).map(move |dy| (x + dx, y + dy)))
                .filter(move |&(nx, ny)| nx != x || ny != y)
                .filter_map(|s| data.get(&s))
                .collect::<HashSet<_>>()
                .iter()
                .map(|n| n.value)
                .collect();

            if numbers.len() > 1 { numbers.iter().product::<i32>() } else { 0 }
        })
        .sum();

    println!("{part2}");
}