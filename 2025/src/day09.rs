type Point = (i64, i64);

fn main() {
    let data: Vec<Point> = include_str!("input")
        .trim()
        .lines()
        .map(|line| line.split(',').map(|x| x.parse::<i64>().unwrap()).collect())
        .map(|circuit: Vec<i64>| (circuit[0], circuit[1]))
        .collect();

    let mut polygon = data.windows(2).map(|w| (w[0], w[1])).collect::<Vec<_>>();
    polygon.push((data[data.len() - 1], data[0]));

    let mut part1_data = data
        .iter()
        .enumerate()
        .flat_map(|(i, c1)| data.iter().skip(i + 1).map(move |c2| (c1, c2)))
        .map(|(c1, c2)| ((*c1, *c2), field(c1, c2)))
        .collect::<Vec<_>>();

    part1_data.sort_by_key(|&(_, field)| field);
    let part1 = part1_data.last().unwrap().1;
    println!("Part 1: {}", part1);


    for &((c1, c2), field) in part1_data.iter().rev() {
        let (x1, y1) = c1; // corner
        let (x2, y2) = c2; // corner
        let c3 = (x1, y2);
        let c4 = (x2, y1);

        // check if all corners of rectangle are inside the polygon
        if !is_point_inside(&polygon, &c3) || !is_point_inside(&polygon, &c4) {
            continue;
        }

        let is_inside = (x1.min(x2)..=x1.max(x2))
            .flat_map(|x| vec![(x, y1), (x, y2)])
            .chain((y1.min(y2)..=y1.max(y2)).flat_map(|y| vec![(x1, y), (x2, y)]))
            .all(|c| is_point_inside(&polygon, &c));

        if is_inside {
            println!("Part 2: {:?}", field);
            break;
        }
    }
}

fn is_point_inside(
    border: &Vec<(Point, Point)>,
    c: &Point,
) -> bool {
    let &(cx, cy) = c;

    // count how many times horizontal half-line crosses the border
    let mut crossings_x_cnt = 0;
    for line in border {
        let &((x1, y1), (x2, y2)) = line;
        if x1 != x2 {
            // only vertical edges cross horizontal ray
            continue;
        }
        if (y1 > cy) != (y2 > cy) && x1 > cx {
            crossings_x_cnt += 1;
        }
    }

    crossings_x_cnt % 2 == 1
}

fn field(c1: &Point, c2: &Point) -> i64 {
    let (x1, y1) = c1;
    let (x2, y2) = c2;
    ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)
}
