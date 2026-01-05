fn main() {
    let data = include_str!("input")
        .trim()
        .split("\n\n")
        .collect::<Vec<_>>();

    let shapes = &data[0..data.len()-1];
    let regions = data[data.len()-1].trim().lines().collect::<Vec<_>>();

    let shapes = shapes.iter().map(|shape| shape.as_bytes().iter().filter(|&&c| c == b'#').count()).collect::<Vec<_>>();

    let part1 = regions.iter().filter(|raw_region| {
        let (size, boxes) = raw_region.split_once(':').unwrap();

        let size = size.trim().split_once('x').unwrap();
        let (w,h) = (size.0.parse::<usize>().unwrap(), size.1.parse::<usize>().unwrap());
        let available_area = w * h;

        let needed_area = boxes
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .enumerate()
            .map(|(i, n)| n * shapes[i])
            .sum::<usize>();

        needed_area <= available_area
    })
    .count();

    println!("Part 1: {}", part1);
}
