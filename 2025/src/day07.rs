use std::collections::{HashMap, HashSet};

fn traverse_beam(
    cache: &mut HashMap<(usize, usize), usize>,
    rows: &Vec<Vec<char>>,
    row_idx: usize,
    curr_idx: usize,
) -> usize {
    if row_idx >= rows.len() {
        return 1;
    }

    if let Some(&res) = cache.get(&(row_idx, curr_idx)) {
        return res;
    }

    if rows[row_idx][curr_idx] == '^' {
        let res = traverse_beam(cache, rows, row_idx + 1, curr_idx - 1)
            + traverse_beam(cache, rows, row_idx + 1, curr_idx + 1);
        cache.insert((row_idx, curr_idx), res);
        res
    } else {
        let res = traverse_beam(cache, rows, row_idx + 1, curr_idx);
        cache.insert((row_idx, curr_idx), res);
        res
    }
}

fn main() {
    let data: Vec<Vec<_>> = include_str!("input")
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start = data[0].iter().position(|&c| c == 'S').unwrap();

    let part1 = data
        .iter()
        .step_by(2)
        .skip(1)
        .fold((HashSet::from([start]), 0), |(acc, mut split), line| {
            let mut new_flow = acc.clone();

            for beam_pos in acc {
                if line[beam_pos] == '^' {
                    split += 1;
                    new_flow.remove(&beam_pos);
                    new_flow.insert(beam_pos - 1);
                    new_flow.insert(beam_pos + 1);
                }
            }

            (new_flow, split)
        })
        .1;

    println!("part1: {:?}", part1);

    let mut cache = HashMap::new();
    let part2 = traverse_beam(&mut cache, &data, 1, start);
    println!("part2: {:?}", part2);
}
