fn check(numbers: &Vec<u64>, idx: usize, acc: u64, expected: u64, part1: bool) -> u64 {
    if expected == acc && idx == numbers.len() {
        return expected;
    }

    if idx >= numbers.len() || acc > expected {
        return 0;
    }
    
    if check(numbers, idx + 1, acc * numbers[idx], expected, part1) > 0 {
        return expected;
    }

    if check(numbers, idx + 1, acc + numbers[idx], expected, part1) > 0 {
        return expected;
    }
    
    if !part1 {
        let mut tmp = numbers[idx];
        let mut new_acc = acc;
        while tmp > 0 {
            new_acc *= 10;
            tmp /= 10;
        }

        if check(numbers, idx + 1, new_acc + numbers[idx], expected, part1) > 0 {
            return expected;
        }
    }
    
    0
}

fn main() {
    let data = include_str!("./input.txt")
        .trim()
        .lines()
        .map(|line| {
            let (target, numbers) = line.split_once(": ").unwrap();
            let target = target.parse().unwrap();
            let numbers = numbers
                .trim()
                .split(" ")
                .map(|number| number.parse().unwrap())
                .collect();
            (target, numbers)
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    let part1 = data
        .iter()
        .map(|(target, numbers)| check(numbers, 1, numbers[0], *target, true))
        .sum::<u64>();
    println!("{}", part1);

    let part2 = data
        .iter()
        .map(|(target, numbers)| check(numbers, 1, numbers[0], *target, false))
        .sum::<u64>();
    println!("{}", part2);
}
