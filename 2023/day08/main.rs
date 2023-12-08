use std::collections::{HashMap};
use num::integer::lcm;

#[derive(Debug)]
struct Trace {
    current: &'static str,
    visited: HashMap<(usize, &'static str), u64>,
    cycle_offset: u64,
    cycle_len: u64,
    target_node: u64,
    has_cycle: bool,
}

impl Trace {
    fn new(k: &'static str) -> Trace {
        Trace {
            current: k,
            cycle_offset: 0,
            cycle_len: 0,
            visited: HashMap::new(),
            target_node: 0,
            has_cycle: false,
        }
    }
}

fn main() {
    let (moves_raw, map_raw) = include_str!("./input").split_once("\n\n").unwrap();

    let moves: Vec<_> = moves_raw.trim().chars().map(|x|  [0,1][(x!='L') as usize]).collect();
    let map: HashMap<_, _> = map_raw.lines().map(|line| {
        let cur = &line[0..3];
        let l = &line[7..10];
        let r = &line[12..15];
        (cur, [l, r])
    })
        .collect();

    // let mut part1 = 0;
    // let mut midx = 0;
    // let mut current = "AAA";
    // while current != "ZZZ" {
    //     let m = moves[midx];
    //     part1 += 1;
    //     let opt = map.get(&current).unwrap();
    //     current = opt[m];
    //
    //     midx += 1;
    //     midx %= moves.len();
    // }


    // println!("part1 {part1}");

    let mut midx = 0;
    let mut steps = 0;
    let mut current_nodes: Vec<Trace> = map.keys()
        .filter(|&k| k.ends_with('A'))
        .map(|&k| Trace::new(k))
        .collect();

    while current_nodes.iter().any(|trace| !trace.has_cycle) {
        let m = moves[midx]; // 0 or 1

        current_nodes.iter_mut()
            .filter(|t| !t.has_cycle)
            .for_each(|t| {
                if t.current.ends_with('Z') {
                    t.target_node = steps;
                }

                let was_there = t.visited.get(&(midx, t.current));
                if let Some(&steps_from_start) = was_there{
                    t.has_cycle = true;
                    t.cycle_offset = steps_from_start;
                    t.cycle_len = steps - t.cycle_offset;
                    t.target_node = t.target_node - t.cycle_offset;
                } else {
                    t.visited.insert((midx, t.current), steps);
                }

                let opt = map.get(t.current).unwrap();
                t.current = opt[m];
            });

        // if all nodes are on 'Z' - finish

        midx += 1;
        midx %= moves.len();
        steps += 1;
    }

    // initial equations
    for t in &current_nodes {
        println!("(s-{}) % {} = {}", t.cycle_offset, t.cycle_len, t.target_node);
    }

    // after transformation
    for t in &current_nodes {
        println!("s % {} = {}", t.cycle_len, (t.target_node +t.cycle_offset) % t.cycle_len);
    }

    for t in &current_nodes {
        if let Some(_) = t.visited.get(&(0, "AAA")) {
            println!("part1: {}", t.target_node+t.cycle_offset);
        }
    }

    // it is simple lcm
    let mut part2 = 1;
    for t in &current_nodes {
        part2 = lcm(part2, t.cycle_len);
    }

    println!("part2: {part2}");
}
