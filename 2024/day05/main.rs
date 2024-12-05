use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn main() {
    let (rules, updates) = include_str!("input.txt").split_once("\n\n").unwrap();
    let rules_list: Vec<(u64, u64)> = rules
        .trim()
        .lines()
        .map(|s| {
            let (a, b) = s.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();
    rules_list.iter().for_each(|&(r1, r2)| {
        rules.entry(r1).or_insert_with(HashSet::new).insert(r2);
    });

    let updates: Vec<Vec<u64>> = updates
        .trim()
        .lines()
        .map(|s| s.split(',').map(|x| x.parse::<u64>().unwrap()).collect())
        .collect();

    let (ordered, unordered): (Vec<_>, Vec<_>) = updates
        .iter()
        .map(|update| {
            let ordered =
                update.is_sorted_by(|v1, v2| rules.get(&v1).map_or(false, |set| set.contains(v2)));
            match ordered {
                true => Ok(update),
                false => Err(update),
            }
        })
        .partition_result();

    let part1: u64 = ordered.iter().map(|update| update[update.len() / 2]).sum();

    let part2: u64 = unordered
        .clone()
        .iter_mut()
        .map(|update| {
            update
                .iter()
                .sorted_by(|v1, v2| {
                    let ordered = rules.get(&v1).map_or(false, |set| set.contains(v2));
                    match ordered {
                        false => Ordering::Greater,
                        true => Ordering::Less,
                    }
                })
                .skip(update.len() / 2)
                .next()
                .unwrap()
        })
        .sum();

    println!("{part1}");
    println!("{part2}");
}
