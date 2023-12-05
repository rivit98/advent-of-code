use std::cmp::{min};
use std::ops::Range;

fn intersection(r1: &Range<u64>, r2: &Range<u64>) -> Option<Range<u64>> {
    let start = r1.start.max(r2.start);
    let end = r1.end.min(r2.end);

    if start < end {
        return Some(start..end)
    }

    return None
}

fn main() {
    let data: Vec<&str>  =include_str!("./t")
        .split("\n\n")
        .collect();

    let seeds: Vec<u64> = data[0].splitn(2, |c| c == ':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|token| token.parse::<u64>().unwrap())
        .collect();

    let stages: Vec<Vec<(Range<u64>, Range<u64>, u64)>> = data[1..].iter()
        .map(|group| {
            group.lines()
                .skip(1)
                .map(|line| {
                    let range: Vec<u64> = line.split_ascii_whitespace().map(|token| token.parse::<u64>().unwrap()).collect();
                    (range[0]..range[0]+range[2], range[1]..range[1]+range[2], range[2])
                })
                .collect()
        })
        .collect();



    let current_stages: Vec<(Range<u64>, u64)> = vec![];
    for stage_pair in stages.windows(2) {
        let stage1 = &stage_pair[0];
        let stage2 = &stage_pair[1];

        // what if intersection is not full?
        for (dest, _, _) in stage1 {
            for (dest2, src2, _) in stage2 {
                println!("{dest:?} -> {src2:?} -> {dest2:?}");
                let a = intersection(dest, src2);

                match a {
                    Some(intersection) => {
                        let is = intersection.start;
                        let ie = intersection.end;

                        // intersection can be directly mapped to dest2
                        let off = is - dest.start;
                        let len = ie - is;
                        let mapped_range = dest2.start + off .. dest2.start + off + len;

                        println!(" intersection {intersection:?} |  {off} {len} {mapped_range:?}");
                        let left_rest = dest.start .. intersection.start;
                        let right_rest = intersection.end .. dest.end;
                        let left_cnt = left_rest.end - left_rest.start;
                        let right_cnt = right_rest.end - right_rest.start;
                        println!(" rest: {left_rest:?} | {right_rest:?}");
                        // if not mapped then what?
                    },
                    None => {
                        // if no mapping then skip totally
                    }
                };
            }
            println!("");
        }
        // break;
        println!("");
    }

}