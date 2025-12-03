fn main() {
    let input = include_str!("input01")
        .trim()
        .lines()
        .map(|line| {
            let sign: i32 = match line.as_bytes()[0] {
                b'L' => -1,
                b'R' => 1,
                _ => panic!("Invalid direction"),
            };

            let distance: i32 = line[1..].parse().unwrap();
            sign * distance
        })
        .collect::<Vec<_>>();

    let part1 = input.iter().scan(50, |state, item| {
        *state = (*state + item).rem_euclid(100);
        Some(*state)
    })
        .filter(|&x| x == 0)
        .count();

    println!("Part 1: {}", part1);

    let part2 = input.iter().fold((50, 0), |(pos, count), &item| {
        let new_pos = (pos + item).rem_euclid(100);
        let mut clicks = item.abs() / 100;

        clicks += if new_pos == 0 {
            1
        } else if pos != 0 && (
            (item > 0 && new_pos <= pos) ||
                (item < 0 && new_pos >= pos)
        ) {
            1
        } else {
            0
        };

        (new_pos, count + clicks)
    }).1;

    println!("Part 2: {}", part2);
}
