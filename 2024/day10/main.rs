use std::collections::HashSet;

fn dfs(data: &Vec<Vec<u8>>, size: &(i64, i64), node: &(i64, i64), visited: &mut HashSet<(i64, i64)>, reachable: &mut HashSet<(i64, i64)>) -> u64 {
    let (x,y) = *node;
    let cv = data[x as usize][y as usize] as i64;
    if cv == 9 {
        reachable.insert(*node);
        return 1
    }
    
    if visited.contains(&node) {
        return 0;
    }
    visited.insert(*node);

    let dirs: Vec<(i64, i64)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut c = 0;
    for (dx, dy) in dirs {
        let (nx, ny) = (x+dx, y+dy);
        if nx < 0 || nx >= size.0 || ny < 0 || ny >= size.1 {
            continue;
        }
        
        let nv = data[nx as usize][ny as usize] as i64;
        if nv - cv != 1 {
            continue;
        }

        c += dfs(data, size, &(nx, ny), visited, reachable);
    }
    visited.remove(node);
    c
}

fn main() {
    let data: Vec<Vec<u8>> = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .filter(|line| line.len() > 1)
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(&|(j, v)| v - '0' as u8)
                .collect()
        })
        .collect();

    let maxy = data.len() as i64;
    let maxx = data[0].len() as i64;
    let size  = (maxx, maxy);
    let starts =  (0..maxx).flat_map(|i| (0..maxy).map(move |j| (i, j))).filter(|&(x, y)| data[x as usize][y as usize] == 0).collect::<HashSet<_>>();
    
    let part1: usize = starts.iter().map(|s| {
        let mut visited = HashSet::new();
        let mut reachable = HashSet::new();
        dfs(&data, &size, s, &mut visited, &mut reachable);
        reachable.len()
    }).sum();

    println!("part 1: {}", part1);

    let part2: usize = starts.iter().map(|s| {
        let mut visited = HashSet::new();
        let mut reachable = HashSet::new();
        dfs(&data, &size, s, &mut visited, &mut reachable) as usize
    }).sum();

    println!("part 2: {}", part2);
}
