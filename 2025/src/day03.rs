
fn joltage(row: &Vec<u64>, to_enable: usize) -> u64 {
    let rlen = row.len();
    let mut res = 0;

    let mut last_idx = 0;
    for i in 0..to_enable {
        let right_cap = rlen - to_enable + 1 + i;
        let &(idx, v) = &row[last_idx..right_cap]
            .iter()
            .enumerate()
            .max_by_key(|&(idx, &v)| v * 1000 + (rlen-idx) as u64)
            .unwrap();

        res = 10 * res + v;
        last_idx += idx + 1;
    }
    res
}

fn main() {
    let input: Vec<Vec<u64>> = include_bytes!("input01")
        .split(|x| *x == b'\n')
        .map(|x| x.iter().map(|x| (*x - b'0') as u64).collect())
        .collect();

    let part1: u64 = input
        .iter()
        .map(|row| joltage(row, 2))
        .sum();

    println!("Part 1: {}", part1);

    let part2: u64 = input
        .iter()
        .map(|row| joltage(row, 12))
        .sum();

    println!("Part 2: {}", part2);
}
