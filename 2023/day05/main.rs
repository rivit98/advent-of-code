use std::ops::Range;


type R = Range<i64>;

fn intersection(r1: &R, r2: &R) -> Option<R> {
    let start = r1.start.max(r2.start);
    let end = r1.end.min(r2.end);

    if start < end {
        return Some(start..end);
    }

    return None;
}

fn rec(stages: &Vec<Vec<(R,R)>>, current: &R, idx: usize) -> i64 {
    if idx >= stages.len() {
        return current.start;
    }

    let mut to_check = vec![];
    let mut unused = vec![current.clone()];

    for (src, dst) in &stages[idx] {
        let mut new_unused = vec![];

        while !unused.is_empty() {
            let u = unused.pop().unwrap().clone();
            let diff: i64 = dst.start - src.start;
            match intersection(&src, &u) {
                Some(inter) => {
                    to_check.push(inter.start+diff..inter.end+diff);
                    new_unused.push(u.start..inter.start);
                    new_unused.push(inter.end..u.end);
                },
                None => new_unused.push(u.clone())
            }
        }

        unused = new_unused.into_iter().filter(|r| r.start != r.end).collect();
    }

    unused.iter().chain(&to_check).map(|r| rec(stages, &r, idx + 1)).min().unwrap()
}


fn main() {
    let data: Vec<&str> = include_str!("./input")
        .split("\n\n")
        .collect();

    let seeds: Vec<i64> = data[0].splitn(2, |c| c == ':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|token| token.parse::<i64>().unwrap())
        .collect();

    let stages: Vec<Vec<(R,R)>> = data[1..].iter()
        .map(|group| {
            group.lines()
                .skip(1)
                .map(|line| {
                    let range: Vec<i64> = line.split_ascii_whitespace().map(|token| token.parse::<i64>().unwrap()).collect();
                    (range[1]..range[1] + range[2], range[0]..range[0] + range[2]) // reverse order so mapping is src->dest
                })
                .collect()
        })
        .collect();

    println!("{}", seeds.iter()
        .map(|&seed| seed..seed+1)
        .map(|seed| rec(&stages, &seed, 0))
        .min()
        .unwrap()
    );

    println!("{}", seeds.chunks(2)
        .map(|chunk| chunk[0]..chunk[0]+chunk[1])
        .map(|seed| rec(&stages, &seed, 0))
        .min()
        .unwrap()
    );
}