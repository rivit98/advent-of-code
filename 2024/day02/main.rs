use itertools::Itertools;

fn check(row: &Vec<i64>) -> bool {
    check_wrapper(row.iter())
}

fn check_wrapper<'a, I>(iter: I) -> bool
where
    I: Iterator<Item = &'a i64>,
{
    let mut direction = 0;

    for (x,y) in iter.tuple_windows() {
        let d = x-y;
        if direction >= 0 && d >= 1 && d <= 3 {
            direction = 1
        }  else if direction <= 0 && d >= -3 && d <= -1 {
            direction = -1
        } else{
            return false
        }
    }

    true
}

fn main() {
    let data: Vec<Vec<i64>> = include_str!("./input.txt")
        .trim()
        .lines()
        .map(|line|
            line
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect()
        )
    .collect();

    let part1 = data.iter()
        .map(check)
        .filter(|&x| x)
        .count();

    println!("part 1: {}", part1);

    let part2 = data.iter()
        .map(|row| {
            for i in 0..row.len() {
                let chain = row[0..i].iter().chain(row[i+1..row.len()].iter());
                if check_wrapper(chain) {
                    return true
                }
            }
    
            return false
        })
        .filter(|&x| x)
        .count();
    
    println!("part 2: {}", part2);
}