use std::ops::RangeInclusive;

fn merge_intervals(intervals: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();

    for interval in intervals {
        let (start, end) = (*interval.start(), *interval.end());

        if let Some(last) = merged.last_mut() {
            let last_end = *last.end();

            if start <= last_end + 1 {
                let new_end = last_end.max(end);
                *last = (*last.start())..=new_end;
                continue;
            }
        }

        merged.push(start..=end);
    }

    merged
}

fn main() {
    let (mut ranges, ingredients) = include_str!("input")
        .split_once("\n\n")
        .map(|(range, ingredient)| {
            let ranges: Vec<_> = range
                .lines()
                .flat_map(|line| line.split_once('-'))
                .map(|(start, end)| {
                    let start: u64 = start.parse().unwrap();
                    let end: u64 = end.parse().unwrap();
                    start..=end
                })
                .collect();

            let ingredients: Vec<u64> = ingredient
                .lines()
                .map(|line| line.parse().unwrap())
                .collect();

            (ranges, ingredients)
        })
        .unwrap();

    let part1 = ingredients
        .iter()
        .filter(|&&ingredient| ranges.iter().any(|range| range.contains(&ingredient)))
        .count();

    println!("{}", part1);

    ranges.sort_by_key(|r| *r.start());
    let part2: u64 = merge_intervals(ranges)
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum();

    println!("{:?}", part2);
}
