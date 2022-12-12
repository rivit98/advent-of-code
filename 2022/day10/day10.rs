fn main() {
    let program: Vec<&str> = include_str!("input.txt").trim().lines().collect();

    let mut x = 1;
    let mut x_hist = Vec::new();
    x_hist.push(x);
    for instr in program {
        match instr {
            "noop" => (),
            s => {
                let to_add = s.split_once(' ').unwrap().1.parse::<i32>().unwrap();
                x_hist.push(x);
                x += to_add;
            }
        }
        x_hist.push(x);
    }

    let part1: i32 = (0..=5)
        .map(|x| 20 + x * 40)
        .map(|cycle| x_hist[cycle] * cycle as i32)
        .sum();

    println!("part1 {}", part1);

    let mut screen = vec![vec!['.'; 40]; 6];
    for (cycle, &x) in x_hist.iter().enumerate() {
        let sprite = x;
        let idx: usize = cycle / 40;
        let i = (cycle % 40) as i32;
        if i == sprite-1 || i == sprite || i == sprite + 1 {
            screen[idx][i as usize] = '#';
        }
    }

    println!("part2"); // FPGPHFGH
    for row in screen {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}


