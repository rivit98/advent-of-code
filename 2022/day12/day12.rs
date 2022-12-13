use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeMap};
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, c);
    graph.entry(v2).or_insert_with(BTreeMap::new);
}

// https://github.com/TheAlgorithms/Rust/blob/master/src/graph/dijkstra.rs
pub fn dijkstra<V: Ord + Copy, E: Ord + Copy + Add<Output = E>>(
    graph: &Graph<V, E>,
    start: &V,
) -> BTreeMap<V, Option<(V, E)>> {
    let mut ans = BTreeMap::new();
    let mut prio = BinaryHeap::new();

    // start is the special case that doesn't have a predecessor
    ans.insert(*start, None);

    for (new, weight) in &graph[start] {
        ans.insert(*new, Some((*start, *weight)));
        prio.push(Reverse((*weight, new, start)));
    }

    while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
        match ans[new] {
            // what we popped is what is in ans, we'll compute it
            Some((p, d)) if p == *prev && d == dist_new => {}
            // otherwise it's not interesting
            _ => continue,
        }

        for (next, weight) in &graph[new] {
            match ans.get(next) {
                // if ans[next] is a lower dist than the alternative one, we do nothing
                Some(Some((_, dist_next))) if dist_new + *weight >= *dist_next => {}
                // if ans[next] is None then next is start and so the distance won't be changed, it won't be added again in prio
                Some(None) => {}
                // the new path is shorter, either new was not in ans or it was farther
                _ => {
                    ans.insert(*next, Some((*new, *weight + dist_new)));
                    prio.push(Reverse((*weight + dist_new, next, new)));
                }
            }
        }
    }

    ans
}

fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let data: Vec<Vec<u8>> = include_bytes!("input.txt")
        .split(|b| *b == b'\n')
        .filter(|line| line.len() > 1)
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, v)| match *v as char {
                    'S' => {
                        start = (i, j);
                        0 // 'a'
                    }
                    'E' => {
                        end = (i, j);
                        25 // 'z'
                    }
                    v => v as u8 - 'a' as u8,
                })
                .collect()
        })
        .collect();

    let rows = data.len() as i32;
    let cols = data[0].len() as i32;

    let mut graph: BTreeMap<(usize, usize), BTreeMap<(usize, usize), usize>> = BTreeMap::new();

    let dirs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    (0..rows)
        .flat_map(|i| (0..cols).map(move |j| (i, j)))
        .flat_map(|(i, j)| {
                dirs
                .iter()
                .map(move |dir| (i, j, dir))
        })
        .filter_map(|(i, j, (dx, dy))| {
            let nx = (i as i32) + dx;
            let ny = (j as i32) + dy;
            if nx < 0 || nx >= rows as i32 || ny < 0 || ny >= cols as i32 {
                return None;
            }
            Some(((i as usize, j as usize), (nx as usize, ny as usize)))
        })
        .filter(|((i, j), (nx, ny))| {
            data[*i][*j] + 1 >= data[*nx][*ny]
        })
        .for_each(|((i, j), (nx, ny))| {
            add_edge(&mut graph, (nx, ny), (i, j), 1);
        });

    let ans = dijkstra(&graph, &end);
    println!("part1 {:?}", ans[&start].unwrap().1);

    let part2 = (0..rows)
        .flat_map(|i| (0..cols).map(move |j| (i as usize, j as usize)))
        .filter(|(i, j)| data[*i][*j] == 0)
        .map(|(i, j)| {
            ans.get(&(i,j)).map_or(usize::MAX, |res| res.unwrap().1)
        })
        .min()
        .unwrap();

    println!("part2 {:?}", part2);
}
