


fn main() {
    let mut input: Vec<Vec<u8>> = include_bytes!("input01")
        .split(|x| *x == b'\n')
        .map(|x| x.to_vec())
        .collect();

    let xmax = input[0].len() as i64;
    let ymax = input.len() as i64;

    let part1 = |input: &[Vec<u8>]| {
        (0..xmax)
            .flat_map(|x| (0..ymax).map(move |y| (x, y)) )
            .filter(|&(x, y)| input[y as usize][x as usize] == b'@')
            .filter(|&(x, y)| {
                let neighbors = (-1..=1).flat_map(|x| (-1..=1).map(move |y| (x,y)))
                    .filter(|&(dx, dy)| dx != 0 || dy != 0)
                    .map(|(dx, dy)| (x + dx, y + dy))
                    .filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx < xmax && ny < ymax)
                    .filter(|&(nx, ny)| input[ny as usize][nx as usize] == b'@')
                    .count();

                neighbors < 4
            })
            .collect::<Vec<_>>()
    };

    println!("Part 1: {}", part1(&input).len());

    let mut part2 = 0;
    let mut was_removed = true;
    while was_removed {
        let to_remove = part1(&input);
        to_remove.iter().for_each(|&(x, y)| input[y as usize][x as usize] = b'.');
        part2 += to_remove.len();
        was_removed = !to_remove.is_empty();
    }

    println!("Part 2: {}", part2);
}
