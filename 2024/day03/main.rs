use regex::Regex;
fn main() {
    let data = include_str!("./input.txt").trim();
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let part1: u64 = regex
        .captures_iter(data)
        .map(|c| {
            let a = c.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let b = c.get(2).unwrap().as_str().parse::<u64>().unwrap();
            a * b
        })
        .sum();

    println!("part 1: {}", part1);

    let part2: u64 = data
        .split("do()")
        .map(|dos| {
            println!("dos: {dos}");
            dos.split_once("don't()").unwrap_or_else(|| (dos, "")).0
        })
        .flat_map(|do_line| {
            println!("{do_line}");
            regex
                .captures_iter(do_line)
                .map(|c| {
                    let a = c.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let b = c.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    a * b
                })
        })
        .sum();

    println!("part 2: {}", part2);
}
