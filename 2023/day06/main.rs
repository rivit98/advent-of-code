

fn main() {
    let data: Vec<Vec<u64>> = include_str!("./input").lines()
        .map(|line| line.splitn(2, ':')
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .map(|tok| tok.parse::<u64>().unwrap())
            .collect()
        )
        .collect();

    let time = &data[0];
    let dist = &data[1];

    // (tmax-t) * t > dist
    // delt = tmax^2 - 4 * dist

    let part1: u64 = time.iter().zip(dist).map(|(&t, &d)| {
        let t = t as f64;
        let d = d as f64;
        let delt: f64 = t * t - 4_f64 * d;
        let x1 = (t - delt.sqrt()) / 2_f64;
        let x2 = (t + delt.sqrt()) / 2_f64;
        let winning = x2.ceil() - x1.floor() - 1_f64;
        winning as u64
    }).product();

    println!("part1 {part1}");

    let t = time.iter().fold(String::new(), |acc, x| format!("{acc}{x}")).parse::<f64>().unwrap();
    let d = dist.iter().fold(String::new(), |acc, x| format!("{acc}{x}")).parse::<f64>().unwrap();
    let delt: f64 = t * t - 4_f64 * d;
    let x1 = (t - delt.sqrt()) / 2_f64;
    let x2 = (t + delt.sqrt()) / 2_f64;
    let part2: u64 = (x2.ceil() - x1.floor() - 1_f64) as u64;
    println!("part2 {part2}");
}