use z3::{Config, Context, Optimize, SatResult, ast::Int};

fn fewest_switches(state: u64, buttons: &Vec<u64>, b_idx: usize, used_switches: u64) -> u64 {
    if state == 0 {
        return used_switches;
    }

    if b_idx >= buttons.len() {
        return u64::MAX / 2;
    }

    let p1 = fewest_switches(
        state ^ buttons[b_idx],
        buttons,
        b_idx + 1,
        used_switches + 1,
    );
    let p2 = fewest_switches(state, buttons, b_idx + 1, used_switches);
    p1.min(p2)
}

fn main() {
    let data: Vec<_> = include_str!("input")
        .trim()
        .lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|line| {
            let state: u64 = line[0][1..line[0].len() - 1]
                .chars()
                .enumerate()
                .map(|(idx, c)| if c == '#' { 2_u64.pow(idx as u32) } else { 0 })
                .sum();

            let buttons: Vec<u64> = line[1..line.len() - 1]
                .iter()
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .map(|v| v.parse::<usize>().unwrap())
                        .map(|idx| 2_u64.pow(idx as u32))
                        .sum()
                })
                .collect();

            let jolts: Vec<_> = line[line.len() - 1][1..line[line.len() - 1].len() - 1]
                .split(',')
                .map(|v| v.parse::<u32>().unwrap())
                .collect();

            (state, buttons, jolts)
        })
        .collect();

    let part1: u64 = data
        .iter()
        .map(|(state, buttons, _)| fewest_switches(*state, &buttons, 0, 0))
        .sum();

    println!("part1: {}", part1);

    let part2: i64 = data
        .iter()
        .map(|(_, buttons, jolts)| {
            let opt = Optimize::new();
            let bn = buttons.len();

            let x: Vec<Int> = (0..bn).map(|i| Int::new_const(format!("x{}", i))).collect();
                buttons
                    .iter()
                    .enumerate()
                    .fold(vec![Int::from_i64(0); jolts.len()], |mut acc, (b_idx, b)| {
                        for i in 0..acc.len() {
                            if b & 2_u64.pow(i as u32) != 0 {
                                acc[i] += &x[b_idx];
                            }
                        }
                        acc
                    })
                    .iter()
                    .enumerate()
                    .for_each(|(i, expr)| opt.assert(&expr.eq(&Int::from_i64(jolts[i] as i64))));

            for i in 0..bn {
                opt.assert(&x[i].ge(&Int::from_i64(0)));
            }

            let obj = x.iter().fold(Int::from_i64(0), |acc, xi| acc + xi);
            opt.minimize(&obj);

            match opt.check(&[]) {
                SatResult::Sat => {
                    let model = opt.get_model().unwrap();
                    return x.iter().map(|xi| model.eval(xi, true).unwrap().as_i64().unwrap()).sum::<i64>();
                }
                _ => panic!()
            }
        })
        .sum();

    println!("part2: {}", part2);
}
