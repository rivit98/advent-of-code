

fn predictor(values: &Vec<i64>, part1: bool) -> i64 {
    if values.iter().all(|&x| x == 0) {
        return 0
    }

    let new_values = values
        .windows(2)
        .map(|slice| {
            let a = slice[0];
            let b = slice[1];
            b - a
        })
        .collect();

    if !part1 {
        return values.first().unwrap() - predictor(&new_values, part1);
    }

    return predictor(&new_values, part1) + values.last().unwrap();
}


fn main() {
    let data: Vec<Vec<i64>> = include_str!("./input").trim().lines()
        .map(|line| line.split_whitespace().map(|token| token.parse::<i64>().unwrap()).collect())
        .collect();

    let part1: i64 = data.iter().map(|x| predictor(x, true)).sum();
    println!("part1: {part1}");

    let part2: i64 = data.iter().map(|x| predictor(x, false)).sum();
    println!("part2: {part2}");
}