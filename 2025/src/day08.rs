use std::collections::{HashMap, HashSet};

type Circuit = (i64, i64, i64);

fn part1(circuits: &Vec<Circuit>, closest_pairs: &Vec<((Circuit, Circuit), i64)>) -> usize {
    let mut boxes_association: HashMap<Circuit, usize> = HashMap::from_iter(circuits.iter().enumerate().map(|(i, &c)| (c, i)));
    for ((c1, c2), _) in closest_pairs.iter().take(1000) {
        if let Some(&c1j) = boxes_association.get(&c1) && let Some(&c2j) = boxes_association.get(&c2) {
            if c1j != c2j {
                for value in boxes_association.values_mut() {
                    if *value == c1j {
                        *value = c2j;
                    }
                }
            }
        }
    }

    let mut group_counts = HashMap::new();
    for &group in boxes_association.values() {
        *group_counts.entry(group).or_insert(0) += 1;
    }

    let mut group_sizes = group_counts.values().cloned().collect::<Vec<usize>>();
    group_sizes.sort();
    let part1: usize = group_sizes.iter().rev().take(3).product();
    group_sizes.iter().rev().take(3).product()
}

fn part2(circuits: &Vec<Circuit>, closest_pairs: &Vec<((Circuit, Circuit), i64)>) -> i64 {
    let mut boxes_association: HashMap<Circuit, usize> = HashMap::from_iter(circuits.iter().enumerate().map(|(i, &c)| (c, i)));
    let mut circuit_indexes: HashSet<usize> = HashSet::from_iter(0..circuits.len());
    let mut last_merged = None;
    for ((c1, c2), _) in closest_pairs {
        if let Some(&c1j) = boxes_association.get(&c1) && let Some(&c2j) = boxes_association.get(&c2) {
            if c1j != c2j {
                for value in boxes_association.values_mut() {
                    if *value == c1j {
                        *value = c2j;
                    }
                }
                last_merged = Some((c1, c2));
                circuit_indexes.remove(&c1j);
                if circuit_indexes.len() == 1 {
                    break;
                }
            }
        }
    }

    last_merged.and_then(|(c1, c2)| Some(c1.0 * c2.0)).unwrap_or(0)
}

fn main() {
    let data: Vec<Circuit> = include_str!("input")
        .trim()
        .lines()
        .map(|line| line.split(',').map(|x| x.parse::<i64>().unwrap()).collect())
        .map(|circuit: Vec<i64>| (circuit[0], circuit[1], circuit[2]))
        .collect();

    let mut closest_pairs: Vec<_> = data
        .iter()
        .enumerate()
        .flat_map(&(|(i, c1)| data.iter().skip(i+1).map(move |c2| (c1, c2))))
        .map(|(&c1, &c2)| {
            let (x1, y1, z1) = c1;
            let (x2, y2, z2) = c2;
            let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
            ((c1, c2), dist)
        })
        .collect();

    closest_pairs.sort_by_key(|&(_, dist)| dist);

    println!("Part 1: {}", part1(&data, &closest_pairs));
    println!("Part 2: {}", part2(&data, &closest_pairs));
}
