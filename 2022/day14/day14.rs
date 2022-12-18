use std::collections::{HashSet};

type Point = (i32, i32);

fn main() {
    let mut rocks: HashSet<Point> = HashSet::new();

    let data: Vec<Vec<Point>> = include_str!("input.txt").trim()
        .lines()
        .map(|line|
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(",").unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        )
        .collect();

    data.iter()
        .for_each(|line| {
            line.windows(2).for_each(|chunk| {
                let (x1, y1) = chunk[0];
                let (x2, y2) = chunk[1];

                let (startx, endx) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
                let (starty, endy) = if y1 > y2 { (y2, y1) } else { (y1, y2) };

                (startx..=endx)
                    .flat_map(|x| (starty..=endy).map(move |y| (x, y)))
                    .for_each(|point| { rocks.insert(point); });
            })
        });

    let lowest_rock = rocks.iter()
        .map(|(_, y)| *y)
        .max()
        .unwrap();

    let mut rocks_part1 = rocks.clone();
    let part1 = simulate(&mut rocks_part1, lowest_rock, false);
    println!("part1 {part1}");

    let mut rocks_part2 = rocks.clone();
    let part2 = simulate(&mut rocks_part2, lowest_rock, true);
    println!("part2 {part2}");
}

fn one_round(rocks: &HashSet<Point>, curr_sand: Point, max_y: i32, part1: bool) -> Option<Point> {
    if curr_sand.1 >= max_y + 3 {
        return None;
    }

    let check_fn: Box<dyn Fn(&Point) -> bool> = match part1 {
        false => Box::new(|x| rocks.contains(x)),
        true => Box::new(|x| x.1 >= max_y + 2 || rocks.contains(x)),
    };

    let (cx, cy) = curr_sand;
    let (nx, ny) = (cx, cy + 1);
    if !check_fn(&(nx, ny)) {
        return one_round(rocks, (nx, ny), max_y, part1);
    }

    let (nx, ny) = (cx - 1, cy + 1);
    if !check_fn(&(nx, ny)) {
        return one_round(rocks, (nx, ny), max_y, part1);
    }

    let (nx, ny) = (cx + 1, cy + 1);
    if !check_fn(&(nx, ny)) {
        return one_round(rocks, (nx, ny), max_y, part1);
    }

    Some((cx, cy))
}

fn simulate(rocks: &mut HashSet<Point>, lowest_rock_y: i32, part1: bool) -> usize {
    let mut steps: usize = 0;
    let sand_start = (500, 0);
    let mut curr_sand;

    loop {
        let next = one_round(rocks, sand_start, lowest_rock_y, part1);
        curr_sand = match next {
            None => break,
            Some(v) => v
        };
        rocks.insert(curr_sand);

        steps += 1;
        if curr_sand.1 == 0 {
            break;
        }
    }
    steps
}