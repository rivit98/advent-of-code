use std::cmp::max;
use std::collections::{HashMap, HashSet};

type P = (i64, i64);

fn main() {
    let mut maxx = 0;
    let mut maxy = 0;
    let mut antennas: HashMap<char, Vec<P>> = HashMap::new();
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
                if c != '.' && c!= '#' {
                    antennas.entry(c).or_insert(Vec::new()).push((x, y));
                }
            })
        });

    let part1: HashSet<P> = antennas.iter()
        .flat_map(|(_, a)| {
            a.iter()
                .enumerate()
                .flat_map(|(i, a1)| {
                    a.iter()
                        .skip(i + 1)
                        .flat_map(|a2| {
                            let dx = a2.0 - a1.0;
                            let dy = a2.1 - a1.1;
                            let anode1 = (a2.0 + dx, a2.1 + dy);
                            let anode2 = (a1.0 - dx, a1.1 - dy);
                            vec![anode1, anode2]
                        })
                })
        })
        .filter(|&(x,y)| {
            x >= 0 && y >= 0 && x <= maxx && y <= maxy
        })
        .collect();

    println!("part 1: {}", part1.len());

    let part2: HashSet<P> = antennas.iter()
        .flat_map(|(_, a)| {
            a.iter()
                .enumerate()
                .flat_map(|(i, a1)| {
                    a.iter()
                        .skip(i + 1)
                        .flat_map(|a2| {
                            let mut res: Vec<P> = Vec::new();
                            
                            let dx = a2.0 - a1.0;
                            let dy = a2.1 - a1.1;
                            
                            let mut i = 0;
                            loop {
                                let anode = (a2.0 + (dx*i), a2.1 + (dy*i));
                                if anode.0 >= 0 && anode.1 >= 0 && anode.0 <= maxx && anode.1 <= maxy {
                                    res.push(anode);
                                } else {
                                    break;
                                }

                                i += 1
                            }
                            
                            let mut i = 0;
                            loop {
                                let anode = (a2.0 - (dx*i), a2.1 - (dy*i));
                                if anode.0 >= 0 && anode.1 >= 0 && anode.0 <= maxx && anode.1 <= maxy {
                                    res.push(anode);
                                } else {
                                    break;
                                }

                                i += 1
                            }

                            res
                        })
                })
        })
        .collect();

    println!("part 2: {}", part2.len());
}
