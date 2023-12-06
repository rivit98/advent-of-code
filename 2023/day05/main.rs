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



    let mut current_stages: Vec<Range<u64>> = stages[0].iter()
        .map(|(dest, src, _)| dest.clone())
        .collect();

    println!("{current_stages:?}");

    for stage2 in stages.iter().skip(1) {
        let mut new_current_stages: Vec<Range<u64>> = vec![];
        for dest in current_stages.iter() {
            for (dest2, src2, _) in stage2 {
                println!("{dest:?} -> {src2:?} -> {dest2:?}");
                let a = intersection(dest, src2);

                match a {
                    Some(intersection) => {
                        let is = intersection.start;
                        let ie = intersection.end;

                        // [src2.start; is) [is; ie) [ie; src2.end)
                        let pre = src2.start..is;
                        let pre_len = pre.end - pre.start;
                        let post = ie..src2.end;
                        let post_len = post.end - post.start;

                        println!(" pre: {pre:?}   matched: {intersection:?}   post: {post:?}");
                        if pre_len > 0 {
                            new_current_stages.push(pre);
                        }

                        if post_len > 0 {
                            new_current_stages.push(post);
                        }

                        new_current_stages.push(intersection);

                        let mapping_pre = dest2.start.. dest2.start + pre_len;
                        let mapping = dest2.start + pre_len .. dest2.end - post_len;
                        let mapping_post = dest2.end - post_len .. dest2.end;
                        println!(" mapping_pre: {mapping_pre:?}  mapping: {mapping:?}  mapping_post: {mapping_post:?}")
                    },
                    None => {
                        // if no mapping then skip totally
                    }
                };
            }
            println!("{new_current_stages:?}");
            println!("");
        }
        println!("");
        current_stages = new_current_stages;
    }

}