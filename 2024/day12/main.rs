use std::collections::{HashMap, HashSet};

type P = (i64, i64);

fn dfs(data: &Vec<Vec<u8>>, size: &P, area: &mut HashSet<P>, fence: &mut HashMap<P, Vec<P>>, node: P) {
    if area.contains(&node) {
        return;
    }
    area.insert(node);

    let (x,y) = node;
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (nx, ny) = (x+dx, y+dy);
        if nx < 0 || nx >= size.0 || ny < 0 || ny >= size.1 {
            fence.entry((x, y)).or_insert(Vec::new()).push((dx, dy));
            continue;
        }

        if data[nx as usize][ny as usize] != data[x as usize][y as usize] {
            fence.entry((x, y)).or_insert(Vec::new()).push((dx, dy));
            continue;
        }

        dfs(data, size, area, fence, (nx, ny));
    }
}

fn is_neighbour_fence(fence: &P, cur_dir: &P, pos: &P, pos_dir: &P) -> bool {
    let mut variants = HashMap::new();

    // dir -> ((dx,dy),(dirx,diry))
    variants.insert((0,-1), [((0,0), (1,0)), ((0,0), (-1,0)), ((1,-1), (-1,0)), ((-1,-1), (1,0)), ((1,0), (0,-1)), ((-1,0), (0,-1))]);
    variants.insert((0, 1), [((0,0), (1,0)), ((0,0), (-1,0)), ((1,1), (-1,0)), ((-1,1), (1,0)), ((1,0), (0,1)), ((-1,0), (0,1))]);

    variants.insert((1, 0), [((0,0), (0,1)), ((0,0), (0,-1)), ((1,1), (0,-1)), ((1,-1), (0,1)), ((0,1), (1,0)), ((0,-1), (1,0))]);
    variants.insert((-1,0), [((0,0), (0,1)), ((0,0), (0,-1)), ((-1,-1), (0,1)), ((-1,1), (0,-1)), ((0,1), (-1,0)), ((0,-1), (-1,0))]);


    let (x,y) = fence;
    let (px,py) = pos;
    let (dx,  dy) = (px-x, py-y);

    let possibs = variants.get(cur_dir).unwrap();
    let exists = possibs.iter().find(|&(d, rot)| *d == (dx, dy) && rot == pos_dir);
    exists.is_some()
}

fn fence_walk(pos: P, cur_dir: P, fences: &mut HashMap<P, Vec<P>>) -> u64 {
    let mut pos = pos;
    let mut dir = cur_dir;
    let mut ret = 0;

    let mut was_removed = true;
    while was_removed {
        was_removed = false;

        'outer:
        for dx in [0,1,-1] {
            for dy in [0,1,-1] {
                let (nx, ny) = (pos.0 + dx, pos.1 + dy);
                if let Some(possible_dirs) = fences.get_mut(&(nx,ny)) {
                    for (idx, &pos_dir) in possible_dirs.iter().enumerate() {
                        if is_neighbour_fence(&pos, &dir, &(nx,ny), &pos_dir) {
                            fences.get_mut(&(nx,ny)).unwrap().remove(idx);
                            if pos_dir != dir {
                                ret += 1;
                            }
                            pos = (nx,ny);
                            dir = pos_dir;
                            was_removed = true;
                            break 'outer;
                        }
                    }

                    if fences.get(&(nx,ny)).unwrap().is_empty() {
                        fences.remove(&(nx,ny));
                    }
                }
            }
        }
    }
    if dir != cur_dir {
        ret += 1;
    }

    ret
}

fn main() {
    let data: Vec<Vec<u8>> = include_bytes!("input.txt")
        .trim_ascii()
        .split(|b| *b == b'\n')
        .map(|line| line.iter().cloned().collect())
        .collect();

    let maxy = data.len() as i64;
    let maxx = data[0].len() as i64;

    let transposed: Vec<Vec<_>> = (0..maxx).map(|col| {
        (0..maxy)
            .map(|row| data[row as usize][col as usize])
            .collect()
    }).collect();

    let data = transposed;

    let size  = (maxx, maxy);
    let mut to_visit =  (0..maxx).flat_map(|i| (0..maxy).map(move |j| (i, j))).collect::<HashSet<_>>();

    let mut part1 = 0;
    let mut part2 = 0;
    while to_visit.len() > 0 {
        let mut area = HashSet::new();
        let mut fences = HashMap::new();
        let start = to_visit.iter().next().unwrap(); //add its neighbours
        dfs(&data, &size, &mut area, &mut fences, *start);
        let fence_len = fences.values().map(|v| v.len()).sum::<usize>();

        part1 += area.len() * fence_len;

        while fences.len() > 0 {
            let (start, possible_dirs) = fences.iter_mut().next().unwrap();
            let cur_dir = possible_dirs.remove(0);
            let f = fence_walk(*start, cur_dir, &mut fences);
            part2 += area.len() as u64 * f;
        }

        to_visit.retain(|p| !area.contains(p));
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

