use std::collections::{HashMap, HashSet};

fn to_num(s: &str) -> u64 {
    s.as_bytes()
        .iter()
        .fold(0u64, |acc, &b| (acc << 8) | b as u64)
}


fn dfs(
    graph: &HashMap<u64, Vec<u64>>,
    target_node: u64,
    node: u64,
    visited: &mut HashSet<u64>,
) -> u64 {
    if node == target_node {
        return 1;
    }

    let mut reached = 0;
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                reached += dfs(graph, target_node, neighbor, visited);
                visited.remove(&neighbor);
            }
        }
    }

    reached
}

fn dfs2(
    graph: &HashMap<u64, Vec<u64>>,
    target_node: u64,
    node: u64,
    visited: &mut HashSet<u64>,
    cache: &mut HashMap<(u64, bool, bool), u64>,
) -> u64 {
    let has_fft = visited.contains(&to_num("fft"));
    let has_dac = visited.contains(&to_num("dac"));

    if node == target_node {
        if has_fft && has_dac {
            return 1;
        }
        return 0;
    }

    if let Some(&cached) = cache.get(&(node, has_fft, has_dac)) {
        return cached;
    }

    let mut reached = 0;
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                let res = dfs2(graph, target_node, neighbor, visited, cache);
                reached += res;
                visited.remove(&neighbor);
            }
        }
    }

    cache.insert((node, has_fft, has_dac), reached);
    reached
}

fn main() {
    let mut graph: HashMap<_, Vec<_>> = HashMap::new();
    include_str!("input")
        .trim()
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .for_each(|line| {
            graph.insert(
                to_num(line.0),
                line.1.trim().split(' ').map(to_num).collect(),
            );
        });

    let visited = &mut HashSet::new();
    let start = to_num("you");
    let part1 = dfs(&graph, to_num("out"), start, visited);
    println!("part1 {:?}", part1);


    let start = to_num("svr");
    visited.clear();
    let cache = &mut HashMap::new();
    let part2 = dfs2(&graph, to_num("out"), start, visited, cache);
    println!("part2 {:?}", part2);

}
