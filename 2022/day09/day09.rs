use std::collections::HashSet;

type Point = (i32, i32);

fn main() {
    let moves: Vec<char> = include_str!("input.txt").trim().lines()
    .map(|line| {
        let tok = line.split_once(' ').unwrap();
        (tok.0.chars().nth(0).unwrap(), tok.1.parse::<usize>().unwrap())
    })
    .flat_map(|(mv, amount)| vec![mv; amount])
    .collect();

    let mut visited: Vec<HashSet<Point>> = vec![HashSet::<Point>::from_iter([(0,0)]); 9];
    let mut parts = vec![(0,0); 9];
    let mut head: Point = (0,0);

    for mv in moves.iter() {
        head = match mv {
            'R' => (head.0 + 1, head.1),
            'L' => (head.0 - 1, head.1),
            'D' => (head.0, head.1 - 1),
            'U' => (head.0, head.1 + 1),
            _ => unreachable!()
        };
        
        let mut prev = head;
        for (idx, curr) in parts.iter_mut().enumerate() {
            let distx = (curr.0 - prev.0).abs();
            let disty = (curr.1 - prev.1).abs();
            let dist =  distx + disty;
            
            if dist < 2 || (distx == 1 && disty == 1) {
                // no change, stop iteration
                break;
            }

            // move at most one unit in each direction
            let delta = ((prev.0 - curr.0).clamp(-1, 1), (prev.1 - curr.1).clamp(-1,1));
            *curr = (curr.0 + delta.0, curr.1 + delta.1);
            visited[idx].insert(*curr);
            prev = *curr;
        }
    }

    println!("part1 {}", visited.first().unwrap().len());
    println!("part2 {}", visited.last().unwrap().len());
}

