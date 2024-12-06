use std::cmp::max;
use std::collections::HashSet;

type P = (i64, i64);


fn check_guard_path(walls: &HashSet<P>, mut pos: P, size: P) -> Option<HashSet<P>> {
    let (maxx, maxy) = size;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut direction = 0;
    let mut visited: HashSet<(usize, P)> = HashSet::new();

    loop {
        let (curx, cury) = pos;
        if curx <0 || cury < 0 || curx > maxx || cury > maxy {
            break;
        }
        let is_new = visited.insert((direction, pos));
        if !is_new {
            return None
        }
        
        let (dx, dy) = directions[direction];
        let (nx, ny) = (curx + dx, cury + dy);
        let newpos = (nx, ny);
        if walls.contains(&newpos) {
            direction += 1;
        } else{
            pos = newpos;
        }

        direction %= directions.len();
    }

    let visited=  visited.iter().map(|x| x.1).collect();
    Some(visited)
}

fn main() {
    let mut pos: P = (0, 0);
    let mut walls: HashSet<P> = HashSet::new();
    let mut maxx = 0;
    let mut maxy = 0;
    include_str!("./input.txt")
        .trim()
        .split('\n')
        .enumerate()
        .for_each(|(y, s)| {
            s.trim().chars().enumerate().for_each(|(x, c)| {
                let x = x as i64;
                let y = y as i64;
                maxx = max(maxx, x);
                maxy = max(maxy, y);
                match c {
                    '^' => pos = (x, y),
                    '#' => {
                        walls.insert((x,y));
                        ()
                    }
                    _ => (),
                }
            })
        });
    
    let part1 = check_guard_path(&walls, pos, (maxx, maxy)).unwrap();
    println!("part1: {}", part1.len());
    
    
    let mut part2 = 0;
    // try to lock each pos and check loop
    for (x, y) in part1 {
        walls.insert((x,y));

        let res = check_guard_path(&walls, pos, (maxx, maxy));
        if res.is_none() {
            part2 += 1;
        }
        
        walls.remove(&(x, y));
    }
    
    println!("part2: {}", part2);
}
