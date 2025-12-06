fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let cols = v[0].len();
    (0..cols)
        .map(|i| v.iter().map(|row| row[i].clone()).collect())
        .collect()
}

fn main() {
    let data: Vec<_> = include_str!("input").lines().collect();
    let ops = data.last().unwrap().split_whitespace().collect::<Vec<_>>();
    let numbers = data
        .iter()
        .take(data.len() - 1)
        .map(|line| line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part1: i64 =
        transpose(numbers)
            .iter()
            .zip(ops.iter())
            .map(|(nums, &op)| {
        match op {
            "*" => nums.iter().product::<i64>(),
            "+" => nums.iter().sum(),
            _ => unreachable!(),
        }
    })
    .sum();

    println!("Part 1: {}", part1);

    let numbers = data
        .iter()
        .take(data.len() - 1)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();


    let mut t = transpose(numbers);
    let mut part2 = 0;
    let mut op_idx = 0;
    let mut acc = Vec::new();
    t.push(vec![' ']); // Sentinel row to handle last operation
    for row in &t {
        let s = row.iter().collect::<String>().trim().to_string();
        if s.is_empty() {
            let res: i64 = match ops[op_idx] {
                "*" => acc.iter().product(),
                "+" => acc.iter().sum(),
                _ => unreachable!(),
            };
            part2 += res;
            op_idx += 1;
            acc.clear();
        } else {
            acc.push(s.parse::<i64>().unwrap());
        }
    }

    println!("{:?}", part2);
}
