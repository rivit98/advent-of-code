extern crate interval;

use gcollections::ops::*;

use interval::interval_set::ToIntervalSet;
use interval::{IntervalSet};
use interval::ops::Range;
use gcollections::ops::{Difference};

type Point = (i32, i32);

struct Scanner {
    pos: Point,
    beacon: Point,
}

fn dist(p1: &Point, p2: &Point) -> i32 {
    return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

const Y_LINE: i32 = 2000000;
const MAX_Y_LINE: i32 = 4000000;

fn main() {
    let scanners: Vec<Scanner> = include_str!("input.txt")
        .trim()
        .lines()
        .map(|line| {
            let toks: Vec<i32> = line.chars()
                .filter(|c| c.is_ascii_digit() || *c == ' ' || *c == '-')
                .collect::<String>()
                .split_whitespace()
                .map(|tok| tok.parse::<i32>().unwrap())
                .collect();

            Scanner {
                pos: (toks[0], toks[1]),
                beacon: (toks[2], toks[3]),
            }
        })
        .collect();

    let mut covered: Vec<Point> = Vec::new();
    for sc in &scanners {
        let (sx, _) = sc.pos;
        let b = sc.beacon;
        let dist_to_beacon = dist(&sc.pos, &b);

        let y_dist_to_line = dist(&sc.pos, &(sc.pos.0, Y_LINE));
        let max_x_dist = dist_to_beacon - y_dist_to_line;
        if max_x_dist < 0 {
            continue;
        }

        covered.push((sx- max_x_dist, sx+ max_x_dist));
    }

    covered.sort();
    let interval_set = covered.to_interval_set();

    let part1: u32 = interval_set.iter().map(|&i| i.size()-1).sum();
    println!("part1 {}", part1);


    let mut interval_sets : Vec<IntervalSet<i32>> = Vec::new();
    for _ in 0..=MAX_Y_LINE {
        interval_sets.push(IntervalSet::new(0, MAX_Y_LINE));
    }

    for sc in &scanners {
        let (sx, sy) = sc.pos;
        let b = sc.beacon;
        let dist_to_beacon = dist(&sc.pos, &b);

        for dy in 0..=dist_to_beacon {
            let ny = (sy + dy).clamp(0, MAX_Y_LINE) as usize;
            let ny2 = (sy - dy).clamp(0, MAX_Y_LINE) as usize;

            let ix = sx-dist_to_beacon+dy;
            let iy = sx+dist_to_beacon-dy;
            let interval = IntervalSet::new(ix, iy);
            interval_sets[ny] = interval_sets[ny].difference(&interval);
            interval_sets[ny2] = interval_sets[ny2].difference(&interval);
        }
    }

    let (dbeacon_x, dbeacon_y) = interval_sets.iter()
        .enumerate()
        .filter(|(_, iset)| iset.interval_count() == 1)
        .nth(0)
        .map(|(idx, iset)| (iset.iter().nth(0).unwrap().lower() as u64, idx as u64))
        .unwrap();

    println!("{dbeacon_x} {dbeacon_y}");
    println!("part2: {}", dbeacon_x * (MAX_Y_LINE as u64) + dbeacon_y);
}